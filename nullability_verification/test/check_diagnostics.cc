// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include "nullability_verification/test/check_diagnostics.h"

#include "nullability_verification/pointer_nullability_analysis.h"
#include "nullability_verification/pointer_nullability_diagnosis.h"
#include "clang/Analysis/CFG.h"
#include "third_party/llvm/llvm-project/clang/unittests/Analysis/FlowSensitive/TestingSupport.h"
#include "llvm/Testing/Support/Error.h"
#include "third_party/llvm/llvm-project/third-party/unittest/googletest/include/gtest/gtest.h"

namespace clang {
namespace tidy {
namespace nullability {

bool checkDiagnostics(llvm::StringRef SourceCode) {
  std::vector<CFGElement> Diagnostics;
  PointerNullabilityDiagnoser Diagnoser;
  bool Failed = false;
  EXPECT_THAT_ERROR(
      dataflow::test::checkDataflow<PointerNullabilityAnalysis>(
          dataflow::test::AnalysisInputs<PointerNullabilityAnalysis>(
              SourceCode, ast_matchers::hasName("target"),
              [](ASTContext &ASTCtx, dataflow::Environment &) {
                return PointerNullabilityAnalysis(ASTCtx);
              })
              .withPostVisitCFG([&Diagnostics, &Diagnoser](
                                    ASTContext &Ctx, const CFGElement &Elt,
                                    const dataflow::TransferStateForDiagnostics<
                                        PointerNullabilityLattice> &State) {
                auto EltDiagnostics = Diagnoser.diagnose(&Elt, Ctx, State);
                if (EltDiagnostics.has_value()) {
                  Diagnostics.push_back(EltDiagnostics.value());
                }
              })
              .withASTBuildArgs({"-fsyntax-only", "-std=c++17",
                                 "-Wno-unused-value", "-Wno-nonnull"}),
          [&Diagnostics, &Failed](
              const llvm::DenseMap<unsigned, std::string> &Annotations,
              const dataflow::test::AnalysisOutputs &AnalysisData) {
            // Note: use sorted sets for expected and actual lines to improve
            // readability of the error output in case the test fails.
            std::set<unsigned> ExpectedLines, ActualLines;
            for (const auto &[Line, _] : Annotations) {
              ExpectedLines.insert(Line);
            }
            auto &SrcMgr = AnalysisData.ASTCtx.getSourceManager();
            for (auto Element : Diagnostics) {
              if (std::optional<CFGStmt> stmt = Element.getAs<CFGStmt>()) {
                ActualLines.insert(SrcMgr.getPresumedLineNumber(
                    stmt->getStmt()->getBeginLoc()));
              } else if (std::optional<CFGInitializer> init =
                             Element.getAs<CFGInitializer>()) {
                ActualLines.insert(SrcMgr.getPresumedLineNumber(
                    init->getInitializer()->getSourceLocation()));
              } else {
                ADD_FAILURE() << "this code should not be reached";
              }
            }
            EXPECT_THAT(ActualLines, testing::ContainerEq(ExpectedLines));
            if (ActualLines != ExpectedLines) {
              Failed = true;
            }
          }),
      llvm::Succeeded());
  return !Failed;
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang
