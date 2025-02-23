// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/pointer_nullability_analysis.h"

#include <optional>
#include <string>

#include "absl/log/check.h"
#include "nullability_verification/pointer_nullability.h"
#include "nullability_verification/pointer_nullability_lattice.h"
#include "nullability_verification/pointer_nullability_matchers.h"
#include "clang/AST/ASTContext.h"
#include "clang/AST/ASTDumper.h"
#include "clang/AST/Expr.h"
#include "clang/AST/OperationKinds.h"
#include "clang/AST/Stmt.h"
#include "clang/AST/Type.h"
#include "clang/ASTMatchers/ASTMatchFinder.h"
#include "clang/Analysis/FlowSensitive/CFGMatchSwitch.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"
#include "clang/Basic/LLVM.h"
#include "clang/Basic/Specifiers.h"

namespace clang {
namespace tidy {
namespace nullability {

using ast_matchers::MatchFinder;
using dataflow::BoolValue;
using dataflow::CFGMatchSwitchBuilder;
using dataflow::Environment;
using dataflow::PointerValue;
using dataflow::SkipPast;
using dataflow::TransferState;
using dataflow::Value;

namespace {

std::vector<NullabilityKind> prepend(NullabilityKind Head,
                                     ArrayRef<NullabilityKind> Tail) {
  std::vector<NullabilityKind> Result = {Head};
  Result.insert(Result.end(), Tail.begin(), Tail.end());
  return Result;
}

void computeNullability(const Expr* E,
                        TransferState<PointerNullabilityLattice>& State,
                        std::function<std::vector<NullabilityKind>()> Compute) {
  (void)State.Lattice.insertExprNullabilityIfAbsent(E, [&] {
    auto Nullability = Compute();
    if (unsigned ExpectedSize = countPointersInType(E);
        ExpectedSize != Nullability.size()) {
      // A nullability vector must have one entry per pointer in the type.
      // If this is violated, we probably failed to handle some AST node.
      llvm::dbgs() << "=== Bad computed nullability: ===\n";
      llvm::dbgs() << "Expression: \n";
      dump(E, llvm::dbgs());
      llvm::dbgs() << "\nNullability (" << Nullability.size()
                   << " pointers): " << nullabilityToString(Nullability)
                   << "\n";
      llvm::dbgs() << "\nType (" << ExpectedSize << " pointers): \n";
      dump(exprType(E), llvm::dbgs());
      llvm::dbgs() << "=================================\n";

      // We can't meaningfully interpret the vector, so discard it.
      // TODO: fix all broken cases and upgrade to CHECK or DCHECK or so.
      Nullability.assign(ExpectedSize, NullabilityKind::Unspecified);
    }
    return Nullability;
  });
}

/// Compute the nullability annotation of type `T`, which contains types
/// originally written as a class template type parameter.
///
/// Example:
///
/// \code
///   template <typename F, typename S>
///   struct pair {
///     S *_Nullable getNullablePtrToSecond();
///   };
/// \endcode
///
/// Consider the following member call:
///
/// \code
///   pair<int *, int *_Nonnull> x;
///   x.getNullablePtrToSecond();
/// \endcode
///
/// The class template specialization `x` has the following substitutions:
///
///   F=int *, whose nullability is [_Unspecified]
///   S=int * _Nonnull, whose nullability is [_Nonnull]
///
/// The return type of the member call `x.getNullablePtrToSecond()` is
/// S * _Nullable.
///
/// When we call `substituteNullabilityAnnotationsInClassTemplate` with the type
/// `S * _Nullable` and the `base` node of the member call (in this case, a
/// `DeclRefExpr`), it returns the nullability of the given type after applying
/// substitutions, which in this case is [_Nullable, _Nonnull].
std::vector<NullabilityKind> substituteNullabilityAnnotationsInClassTemplate(
    QualType T, ArrayRef<NullabilityKind> BaseNullabilityAnnotations,
    QualType BaseType) {
  return getNullabilityAnnotationsFromType(
      T,
      [&](const SubstTemplateTypeParmType* ST)
          -> std::optional<std::vector<NullabilityKind>> {
        // The class specialization that is BaseType and owns ST.
        const ClassTemplateSpecializationDecl* Specialization = nullptr;
        if (auto RT = BaseType->getAs<RecordType>())
          Specialization =
              dyn_cast<ClassTemplateSpecializationDecl>(RT->getDecl());
        // TODO: handle nested templates, where associated decl != base type
        // (e.g. PointerNullabilityTest.MemberFunctionTemplateOfTemplateStruct)
        if (!Specialization || Specialization != ST->getAssociatedDecl())
          return std::nullopt;

        unsigned ArgIndex = ST->getIndex();
        auto TemplateArgs = Specialization->getTemplateArgs().asArray();

        unsigned PointerCount = 0;
        for (auto TA : TemplateArgs.take_front(ArgIndex)) {
          PointerCount += countPointersInType(TA);
        }
        unsigned SliceSize = countPointersInType(TemplateArgs[ArgIndex]);
        return BaseNullabilityAnnotations.slice(PointerCount, SliceSize).vec();
      });
}

/// Compute nullability annotations of `T`, which might contain template type
/// variable substitutions bound by the call `CE`.
///
/// Example:
///
/// \code
///   template<typename F, typename S>
///   std::pair<S, F> flip(std::pair<F, S> p);
/// \endcode
///
/// Consider the following CallExpr:
///
/// \code
///   flip<int * _Nonnull, int * _Nullable>(std::make_pair(&x, &y));
/// \endcode
///
/// This CallExpr has the following substitutions:
///   F=int * _Nonnull, whose nullability is [_Nonnull]
///   S=int * _Nullable, whose nullability is [_Nullable]
///
/// The return type of this CallExpr is `std::pair<S, F>`.
///
/// When we call `substituteNullabilityAnnotationsInFunctionTemplate` with the
/// type `std::pair<S, F>` and the above CallExpr, it returns the nullability
/// the given type after applying substitutions, which in this case is
/// [_Nullable, _Nonnull].
std::vector<NullabilityKind> substituteNullabilityAnnotationsInFunctionTemplate(
    QualType T, const CallExpr* CE) {
  return getNullabilityAnnotationsFromType(
      T,
      [&](const SubstTemplateTypeParmType* ST)
          -> std::optional<std::vector<NullabilityKind>> {
        // TODO: Handle calls that use template argument deduction.
        // TODO: Handle nested templates (...->getDepth() > 0).
        if (auto* DRE =
                dyn_cast<DeclRefExpr>(CE->getCallee()->IgnoreImpCasts());
            ST->getReplacedParameter()->getDepth() == 0 &&
            DRE->hasExplicitTemplateArgs()) {
          return getNullabilityAnnotationsFromType(
              DRE->template_arguments()[ST->getIndex()]
                  .getTypeSourceInfo()
                  ->getType());
        }
        return std::nullopt;
      });
}

NullabilityKind getPointerNullability(const Expr* E,
                                      PointerNullabilityAnalysis::Lattice& L) {
  QualType ExprType = E->getType();
  std::optional<NullabilityKind> Nullability = ExprType->getNullability();

  // If the expression's type does not contain nullability information, it may
  // be a template instantiation. Look up the nullability in the
  // `ExprToNullability` map.
  if (Nullability.value_or(NullabilityKind::Unspecified) ==
      NullabilityKind::Unspecified) {
    if (auto MaybeNullability = L.getExprNullability(E)) {
      if (!MaybeNullability->empty()) {
        // Return the nullability of the topmost pointer in the type.
        Nullability = (*MaybeNullability)[0];
      }
    }
  }
  return Nullability.value_or(NullabilityKind::Unspecified);
}

void initPointerFromAnnotations(
    PointerValue& PointerVal, const Expr* E,
    TransferState<PointerNullabilityLattice>& State) {
  NullabilityKind Nullability = getPointerNullability(E, State.Lattice);
  switch (Nullability) {
    case NullabilityKind::NonNull:
      initNotNullPointer(PointerVal, State.Env);
      break;
    case NullabilityKind::Nullable:
      initNullablePointer(PointerVal, State.Env);
      break;
    default:
      initUnknownPointer(PointerVal, State.Env);
  }
}

void transferFlowSensitiveNullPointer(
    const Expr* NullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(NullPointer, State.Env)) {
    initNullPointer(*PointerVal, State.Env);
  }
}

void transferFlowSensitiveNotNullPointer(
    const Expr* NotNullPointer, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(NotNullPointer, State.Env)) {
    initNotNullPointer(*PointerVal, State.Env);
  }
}

void transferFlowSensitivePointer(
    const Expr* PointerExpr, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  if (auto* PointerVal = getPointerValueFromExpr(PointerExpr, State.Env)) {
    initPointerFromAnnotations(*PointerVal, PointerExpr, State);
  }
}

// TODO(b/233582219): Implement promotion of nullability knownness for initially
// unknown pointers when there is evidence that it is nullable, for example
// when the pointer is compared to nullptr, or casted to boolean.
void transferFlowSensitiveNullCheckComparison(
    const BinaryOperator* BinaryOp, const MatchFinder::MatchResult& result,
    TransferState<PointerNullabilityLattice>& State) {
  // Boolean representing the comparison between the two pointer values,
  // automatically created by the dataflow framework.
  auto& PointerComparison =
      *cast<BoolValue>(State.Env.getValue(*BinaryOp, SkipPast::None));

  CHECK(BinaryOp->getOpcode() == BO_EQ || BinaryOp->getOpcode() == BO_NE);
  auto& PointerEQ = BinaryOp->getOpcode() == BO_EQ
                        ? PointerComparison
                        : State.Env.makeNot(PointerComparison);
  auto& PointerNE = BinaryOp->getOpcode() == BO_EQ
                        ? State.Env.makeNot(PointerComparison)
                        : PointerComparison;

  auto* LHS = getPointerValueFromExpr(BinaryOp->getLHS(), State.Env);
  auto* RHS = getPointerValueFromExpr(BinaryOp->getRHS(), State.Env);

  if (!LHS || !RHS) return;

  auto [LHSKnown, LHSNull] = getPointerNullState(*LHS, State.Env);
  auto [RHSKnown, RHSNull] = getPointerNullState(*RHS, State.Env);
  auto& LHSKnownNotNull =
      State.Env.makeAnd(LHSKnown, State.Env.makeNot(LHSNull));
  auto& RHSKnownNotNull =
      State.Env.makeAnd(RHSKnown, State.Env.makeNot(RHSNull));
  auto& LHSKnownNull = State.Env.makeAnd(LHSKnown, LHSNull);
  auto& RHSKnownNull = State.Env.makeAnd(RHSKnown, RHSNull);

  // nullptr == nullptr
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNull, RHSKnownNull), PointerEQ));
  // nullptr != notnull
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNull, RHSKnownNotNull), PointerNE));
  // notnull != nullptr
  State.Env.addToFlowCondition(State.Env.makeImplication(
      State.Env.makeAnd(LHSKnownNotNull, RHSKnownNull), PointerNE));
}

void transferFlowSensitiveNullCheckImplicitCastPtrToBool(
    const Expr* CastExpr, const MatchFinder::MatchResult&,
    TransferState<PointerNullabilityLattice>& State) {
  auto* PointerVal =
      getPointerValueFromExpr(CastExpr->IgnoreImplicit(), State.Env);
  if (!PointerVal) return;

  auto [PointerKnown, PointerNull] =
      getPointerNullState(*PointerVal, State.Env);
  auto& CastExprLoc = State.Env.createStorageLocation(*CastExpr);
  State.Env.setValue(CastExprLoc, State.Env.makeNot(PointerNull));
  State.Env.setStorageLocation(*CastExpr, CastExprLoc);
}

void transferFlowSensitiveCallExpr(
    const CallExpr* CallExpr, const MatchFinder::MatchResult& Result,
    TransferState<PointerNullabilityLattice>& State) {
  auto ReturnType = CallExpr->getType();
  if (!ReturnType->isAnyPointerType()) return;

  auto* PointerVal = getPointerValueFromExpr(CallExpr, State.Env);
  if (!PointerVal) {
    PointerVal = cast<PointerValue>(State.Env.createValue(ReturnType));
    auto& CallExprLoc = State.Env.createStorageLocation(*CallExpr);
    State.Env.setValue(CallExprLoc, *PointerVal);
    State.Env.setStorageLocation(*CallExpr, CallExprLoc);
  }
  initPointerFromAnnotations(*PointerVal, CallExpr, State);
}

void transferNonFlowSensitiveDeclRefExpr(
    const DeclRefExpr* DRE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(DRE, State, [&] {
    return getNullabilityAnnotationsFromType(DRE->getType());
  });
}

void transferNonFlowSensitiveMemberExpr(
    const MemberExpr* ME, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(ME, State, [&]() {
    auto BaseNullability = getNullabilityForChild(ME->getBase(), State);
    QualType MemberType = ME->getType();
    // When a MemberExpr is a part of a member function call
    // (a child of CXXMemberCallExpr), the MemberExpr models a
    // partially-applied member function, which isn't a real C++ construct.
    // The AST does not provide rich type information for such MemberExprs.
    // Instead, the AST specifies a placeholder type, specifically
    // BuiltinType::BoundMember. So we have to look at the type of the member
    // function declaration.
    if (ME->hasPlaceholderType(BuiltinType::BoundMember)) {
      MemberType = ME->getMemberDecl()->getType();
    }
    return substituteNullabilityAnnotationsInClassTemplate(
        MemberType, BaseNullability, ME->getBase()->getType());
  });
}

void transferNonFlowSensitiveMemberCallExpr(
    const CXXMemberCallExpr* MCE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(MCE, State, [&]() {
    return getNullabilityForChild(MCE->getCallee(), State).vec();
  });
}

void transferNonFlowSensitiveCastExpr(
    const CastExpr* CE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  // TODO: Handle casts where the input and output types can have different
  // numbers of pointers, and therefore different nullability. For example, a
  // reinterpret_cast from `int *` to int.
  computeNullability(CE, State, [&]() {
    return getNullabilityForChild(CE->getSubExpr(), State).vec();
  });
}

void transferNonFlowSensitiveMaterializeTemporaryExpr(
    const MaterializeTemporaryExpr* MTE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(MTE, State, [&]() {
    return getNullabilityForChild(MTE->getSubExpr(), State).vec();
  });
}

void transferNonFlowSensitiveCallExpr(
    const CallExpr* CE, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  // TODO: Check CallExpr arguments in the diagnoser against the nullability of
  // parameters.
  computeNullability(CE, State, [&]() {
    return substituteNullabilityAnnotationsInFunctionTemplate(CE->getType(),
                                                              CE);
  });
}

void transferNonFlowSensitiveUnaryOperator(
    const UnaryOperator* UO, const MatchFinder::MatchResult& MR,
    TransferState<PointerNullabilityLattice>& State) {
  computeNullability(UO, State, [&]() -> std::vector<NullabilityKind> {
    switch (UO->getOpcode()) {
      case UO_AddrOf:
        return prepend(NullabilityKind::NonNull,
                       getNullabilityForChild(UO->getSubExpr(), State));
      case UO_Deref:
        return getNullabilityForChild(UO->getSubExpr(), State)
            .drop_front()
            .vec();

      case UO_PostInc:
      case UO_PostDec:
      case UO_PreInc:
      case UO_PreDec:
      case UO_Plus:
      case UO_Minus:
      case UO_Not:
      case UO_LNot:
      case UO_Real:
      case UO_Imag:
      case UO_Extension:
        return getNullabilityForChild(UO->getSubExpr(), State);

      case UO_Coawait:
        // TODO: work out what to do here!
        return unspecifiedNullability(UO);
    }
  });
}

auto buildNonFlowSensitiveTransferer() {
  return CFGMatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      .CaseOfCFGStmt<DeclRefExpr>(ast_matchers::declRefExpr(),
                                  transferNonFlowSensitiveDeclRefExpr)
      .CaseOfCFGStmt<MemberExpr>(ast_matchers::memberExpr(),
                                 transferNonFlowSensitiveMemberExpr)
      .CaseOfCFGStmt<CXXMemberCallExpr>(ast_matchers::cxxMemberCallExpr(),
                                        transferNonFlowSensitiveMemberCallExpr)
      .CaseOfCFGStmt<CastExpr>(ast_matchers::castExpr(),
                               transferNonFlowSensitiveCastExpr)
      .CaseOfCFGStmt<MaterializeTemporaryExpr>(
          ast_matchers::materializeTemporaryExpr(),
          transferNonFlowSensitiveMaterializeTemporaryExpr)
      .CaseOfCFGStmt<CallExpr>(ast_matchers::callExpr(),
                               transferNonFlowSensitiveCallExpr)
      .CaseOfCFGStmt<UnaryOperator>(ast_matchers::unaryOperator(),
                                    transferNonFlowSensitiveUnaryOperator)
      .Build();
}

auto buildFlowSensitiveTransferer() {
  return CFGMatchSwitchBuilder<TransferState<PointerNullabilityLattice>>()
      // Handles initialization of the null states of pointers.
      .CaseOfCFGStmt<Expr>(isCXXThisExpr(), transferFlowSensitiveNotNullPointer)
      .CaseOfCFGStmt<Expr>(isAddrOf(), transferFlowSensitiveNotNullPointer)
      .CaseOfCFGStmt<Expr>(isNullPointerLiteral(),
                           transferFlowSensitiveNullPointer)
      .CaseOfCFGStmt<CallExpr>(isCallExpr(), transferFlowSensitiveCallExpr)
      .CaseOfCFGStmt<Expr>(isPointerExpr(), transferFlowSensitivePointer)
      // Handles comparison between 2 pointers.
      .CaseOfCFGStmt<BinaryOperator>(isPointerCheckBinOp(),
                                     transferFlowSensitiveNullCheckComparison)
      // Handles checking of pointer as boolean.
      .CaseOfCFGStmt<Expr>(isImplicitCastPointerToBool(),
                           transferFlowSensitiveNullCheckImplicitCastPtrToBool)
      .Build();
}
}  // namespace

PointerNullabilityAnalysis::PointerNullabilityAnalysis(ASTContext& Context)
    : DataflowAnalysis<PointerNullabilityAnalysis, PointerNullabilityLattice>(
          Context),
      NonFlowSensitiveTransferer(buildNonFlowSensitiveTransferer()),
      FlowSensitiveTransferer(buildFlowSensitiveTransferer()) {}

void PointerNullabilityAnalysis::transfer(const CFGElement& Elt,
                                          PointerNullabilityLattice& Lattice,
                                          Environment& Env) {
  TransferState<PointerNullabilityLattice> State(Lattice, Env);
  NonFlowSensitiveTransferer(Elt, getASTContext(), State);
  FlowSensitiveTransferer(Elt, getASTContext(), State);
}

BoolValue& mergeBoolValues(BoolValue& Bool1, const Environment& Env1,
                           BoolValue& Bool2, const Environment& Env2,
                           Environment& MergedEnv) {
  if (&Bool1 == &Bool2) {
    return Bool1;
  }

  auto& MergedBool = MergedEnv.makeAtomicBoolValue();

  // If `Bool1` and `Bool2` is constrained to the same true / false value,
  // `MergedBool` can be constrained similarly without needing to consider the
  // path taken - this simplifies the flow condition tracked in `MergedEnv`.
  // Otherwise, information about which path was taken is used to associate
  // `MergedBool` with `Bool1` and `Bool2`.
  if (Env1.flowConditionImplies(Bool1) && Env2.flowConditionImplies(Bool2)) {
    MergedEnv.addToFlowCondition(MergedBool);
  } else if (Env1.flowConditionImplies(Env1.makeNot(Bool1)) &&
             Env2.flowConditionImplies(Env2.makeNot(Bool2))) {
    MergedEnv.addToFlowCondition(MergedEnv.makeNot(MergedBool));
  } else {
    // TODO(b/233582219): Flow conditions are not necessarily mutually
    // exclusive, a fix is in order: https://reviews.llvm.org/D130270. Update
    // this section when the patch is commited.
    auto& FC1 = Env1.getFlowConditionToken();
    auto& FC2 = Env2.getFlowConditionToken();
    MergedEnv.addToFlowCondition(MergedEnv.makeOr(
        MergedEnv.makeAnd(FC1, MergedEnv.makeIff(MergedBool, Bool1)),
        MergedEnv.makeAnd(FC2, MergedEnv.makeIff(MergedBool, Bool2))));
  }
  return MergedBool;
}

bool PointerNullabilityAnalysis::merge(QualType Type, const Value& Val1,
                                       const Environment& Env1,
                                       const Value& Val2,
                                       const Environment& Env2,
                                       Value& MergedVal,
                                       Environment& MergedEnv) {
  if (!Type->isAnyPointerType()) {
    return false;
  }

  auto [Known1, Null1] = getPointerNullState(cast<PointerValue>(Val1), Env1);
  auto [Known2, Null2] = getPointerNullState(cast<PointerValue>(Val2), Env2);

  auto& Known = mergeBoolValues(Known1, Env1, Known2, Env2, MergedEnv);
  auto& Null = mergeBoolValues(Null1, Env1, Null2, Env2, MergedEnv);

  initPointerNullState(cast<PointerValue>(MergedVal), MergedEnv, &Known, &Null);

  return true;
}
}  // namespace nullability
}  // namespace tidy
}  // namespace clang
