// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(arbitrary_self_types, custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![no_std]
#![allow(improper_ctypes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct Nontrivial {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl ::ctor::CtorNew<i32> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        field,
                    );
                },
            )
        }
    }
}
impl ::ctor::CtorNew<(i32,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(i32, i32)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        field,
                        unused,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<i32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEi(self, __param_0);
        }
    }
}

impl ::ctor::Assign<f32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: f32) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialaSEf(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        self,
                        __param_0,
                    );
                }
            ));
        }
    }
}

impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn Unqualified<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial11UnqualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn ConstQualified<'a>(&'a self) {
        unsafe { crate::detail::__rust_thunk___ZNK10Nontrivial14ConstQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn LvalueRefQualified<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn ConstLvalueRefQualified<'a>(&'a self) {
        unsafe { crate::detail::__rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn RvalueRefQualified<'a>(self: ::ctor::RvalueReference<'a, Self>) {
        unsafe { crate::detail::__rust_thunk___ZNO10Nontrivial18RvalueRefQualifiedEv(self) }
    }
}

impl Nontrivial {
    #[inline(always)]
    pub fn ConstRvalueRefQualified<'a>(self: ::ctor::ConstRvalueReference<'a, Self>) {
        unsafe { crate::detail::__rust_thunk___ZNKO10Nontrivial23ConstRvalueRefQualifiedEv(self) }
    }
}

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct NontrivialInline {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialInline"),
    crate::NontrivialInline
);

impl ::ctor::CtorNew<()> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl ::ctor::CtorNew<i32> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        field,
                    );
                },
            )
        }
    }
}
impl ::ctor::CtorNew<(i32,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

impl ::ctor::CtorNew<(i32, i32)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        field,
                        unused,
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::Assign<i32> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEi(self, __param_0);
        }
    }
}

impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self)
    }
}

impl NontrivialInline {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct NontrivialMembers {
    pub nontrivial_member: ::core::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialMembers"),
    crate::NontrivialMembers
);

impl ::ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        __param_0,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl ::ctor::PinnedDrop for NontrivialMembers {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSEOS_(self, __param_0);
        }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
#[repr(C)]
pub struct NontrivialUnpin {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialUnpin"),
    crate::NontrivialUnpin
);

impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

impl From<i32> for NontrivialUnpin {
    #[inline(always)]
    fn from(field: i32) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ei(&mut tmp, field);
            tmp.assume_init()
        }
    }
}

// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// More than 1 constructor parameter is not supported yet

impl Clone for NontrivialUnpin {
    #[inline(always)]
    fn clone<'b>(&'b self) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1ERKS_(&mut tmp, self);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> From<::ctor::RvalueReference<'b, crate::Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>) -> Self {
        let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

impl<'b> ::ctor::UnpinAssign<&'b Self> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSERKS_(self, __param_0);
        }
    }
}

impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSEOS_(self, __param_0);
        }
    }
}

impl ::ctor::UnpinAssign<i32> for NontrivialUnpin {
    #[inline(always)]
    fn unpin_assign<'a>(&'a mut self, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinaSEi(self, __param_0);
        }
    }
}

impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

impl NontrivialUnpin {
    #[inline(always)]
    pub fn MemberFunction<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(self) }
    }
}

#[inline(always)]
pub fn TakesByValue(
    nontrivial: impl ::ctor::Ctor<Output = crate::Nontrivial>,
) -> impl ::ctor::Ctor<Output = crate::Nontrivial> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nontrivial>>| {
                crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(
                    ::core::pin::Pin::into_inner_unchecked(dest),
                    ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
                );
            },
        )
    }
}

#[inline(always)]
pub fn TakesByValueInline(
    nontrivial: impl ::ctor::Ctor<Output = crate::NontrivialInline>,
) -> impl ::ctor::Ctor<Output = crate::NontrivialInline> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::core::pin::Pin<
                &mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            >| {
                crate::detail::__rust_thunk___Z18TakesByValueInline16NontrivialInline(
                    ::core::pin::Pin::into_inner_unchecked(dest),
                    ::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
                );
            },
        )
    }
}

#[inline(always)]
pub fn TakesByValueUnpin(mut nontrivial: crate::NontrivialUnpin) -> crate::NontrivialUnpin {
    unsafe {
        let mut __return = ::core::mem::MaybeUninit::<crate::NontrivialUnpin>::uninit();
        crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            &mut __return,
            &mut nontrivial,
        );
        __return.assume_init()
    }
}

#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: ::core::pin::Pin<&'a mut crate::Nontrivial>,
) -> ::core::pin::Pin<&'a mut crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByReference<'a>(
    nontrivial: &'a mut crate::NontrivialUnpin,
) -> &'a mut crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z21TakesUnpinByReferenceR15NontrivialUnpin(nontrivial) }
}

#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a crate::Nontrivial) -> &'a crate::Nontrivial {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByConstReference<'a>(
    nontrivial: &'a crate::NontrivialUnpin,
) -> &'a crate::NontrivialUnpin {
    unsafe {
        crate::detail::__rust_thunk___Z26TakesUnpinByConstReferenceRK15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByRvalueReference<'a>(
    nontrivial: ::ctor::RvalueReference<'a, crate::Nontrivial>,
) -> ::ctor::RvalueReference<'a, crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z22TakesByRvalueReferenceO10Nontrivial(nontrivial) }
}

#[inline(always)]
pub fn TakesUnpinByRvalueReference<'a>(
    nontrivial: ::ctor::RvalueReference<'a, crate::NontrivialUnpin>,
) -> ::ctor::RvalueReference<'a, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin(nontrivial)
    }
}

#[inline(always)]
pub fn TakesByConstRvalueReference<'a>(
    nontrivial: ::ctor::ConstRvalueReference<'a, crate::Nontrivial>,
) -> ::ctor::ConstRvalueReference<'a, crate::Nontrivial> {
    unsafe {
        crate::detail::__rust_thunk___Z27TakesByConstRvalueReferenceOK10Nontrivial(nontrivial)
    }
}

#[inline(always)]
pub fn TakesUnpinByConstRvalueReference<'a>(
    nontrivial: ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin>,
) -> ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin> {
    unsafe {
        crate::detail::__rust_thunk___Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin(
            nontrivial,
        )
    }
}

/// Finally, testing for strange by-value APIs.
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct NontrivialByValue {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialByValue"),
    crate::NontrivialByValue
);

impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let other = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueC1ERKS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        other,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let other = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueC1EOS_(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        other,
                    );
                },
            )
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

impl<'b> ::ctor::Assign<&'b Self> for NontrivialByValue {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSERKS_(self, other);
        }
    }
}

impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialByValue {
    #[inline(always)]
    fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, other: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSEOS_(self, other);
        }
    }
}

impl<'other> ::ctor::Assign<::ctor::RvalueReference<'other, crate::Nontrivial>>
    for NontrivialByValue
{
    #[inline(always)]
    fn assign<'a>(
        self: ::core::pin::Pin<&'a mut Self>,
        other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
    ) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                        self,
                        other,
                    );
                }
            ));
        }
    }
}

// Error while generating bindings for item 'NontrivialByValue::operator==':
// operator== where lhs operand is not record nor const reference to record

#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct Nonmovable {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nonmovable"), crate::Nonmovable);

impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NonmovableC1Ev(
                        ::core::pin::Pin::into_inner_unchecked(dest),
                    );
                },
            )
        }
    }
}

impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self)
    }
}

impl Nonmovable {
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::core::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nonmovable14MemberFunctionEv(self) }
    }
}

// Error while generating bindings for item 'TakesNonmovableByValue':
// Non-movable, non-trivial_abi type 'crate :: Nonmovable' is not supported by value as parameter #0

#[inline(always)]
pub fn ReturnsNonmovableByValue() -> impl ::ctor::Ctor<Output = crate::Nonmovable> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<crate::Nonmovable>>| {
                crate::detail::__rust_thunk___Z24ReturnsNonmovableByValuev(
                    ::core::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ei<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Eii<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
            unused: i32,
        );
        #[link_name = "_ZN10NontrivialC1ERKS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialaSERKS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEi"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEi<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: i32,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEf<'a>(
            __return: &mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: f32,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZN10Nontrivial11UnqualifiedEv"]
        pub(crate) fn __rust_thunk___ZN10Nontrivial11UnqualifiedEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZNK10Nontrivial14ConstQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNK10Nontrivial14ConstQualifiedEv<'a>(
            __this: &'a crate::Nontrivial,
        );
        #[link_name = "_ZNR10Nontrivial18LvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNR10Nontrivial18LvalueRefQualifiedEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZNKR10Nontrivial23ConstLvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKR10Nontrivial23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::Nontrivial,
        );
        #[link_name = "_ZNO10Nontrivial18RvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNO10Nontrivial18RvalueRefQualifiedEv<'a>(
            __this: ::ctor::RvalueReference<'a, crate::Nontrivial>,
        );
        #[link_name = "_ZNKO10Nontrivial23ConstRvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKO10Nontrivial23ConstRvalueRefQualifiedEv<'a>(
            __this: ::ctor::ConstRvalueReference<'a, crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ei<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Eii<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
            unused: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSEi<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: i32,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialMembers>;
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialMembers>;
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1Ei"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1Ei<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialUnpin>,
            field: i32,
        );
        #[link_name = "_ZN15NontrivialUnpinC1ERKS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: &'b crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EOS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinaSERKS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinaSERKS_<'a, 'b>(
            __this: &'a mut crate::NontrivialUnpin,
            __param_0: &'b crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinaSEOS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinaSEOS_<'a, 'b>(
            __this: &'a mut crate::NontrivialUnpin,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinaSEi"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinaSEi<'a>(
            __this: &'a mut crate::NontrivialUnpin,
            __param_0: i32,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        pub(crate) fn __rust_thunk___Z12TakesByValue10Nontrivial(
            __return: &mut ::core::mem::MaybeUninit<crate::Nontrivial>,
            nontrivial: &mut crate::Nontrivial,
        );
        pub(crate) fn __rust_thunk___Z18TakesByValueInline16NontrivialInline(
            __return: &mut ::core::mem::MaybeUninit<crate::NontrivialInline>,
            nontrivial: &mut crate::NontrivialInline,
        );
        pub(crate) fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            __return: &mut ::core::mem::MaybeUninit<crate::NontrivialUnpin>,
            nontrivial: &mut crate::NontrivialUnpin,
        );
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: ::core::pin::Pin<&'a mut crate::Nontrivial>,
        ) -> ::core::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_Z21TakesUnpinByReferenceR15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z21TakesUnpinByReferenceR15NontrivialUnpin<'a>(
            nontrivial: &'a mut crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a crate::Nontrivial,
        ) -> &'a crate::Nontrivial;
        #[link_name = "_Z26TakesUnpinByConstReferenceRK15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z26TakesUnpinByConstReferenceRK15NontrivialUnpin<'a>(
            nontrivial: &'a crate::NontrivialUnpin,
        ) -> &'a crate::NontrivialUnpin;
        #[link_name = "_Z22TakesByRvalueReferenceO10Nontrivial"]
        pub(crate) fn __rust_thunk___Z22TakesByRvalueReferenceO10Nontrivial<'a>(
            nontrivial: ::ctor::RvalueReference<'a, crate::Nontrivial>,
        ) -> ::ctor::RvalueReference<'a, crate::Nontrivial>;
        #[link_name = "_Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z27TakesUnpinByRvalueReferenceO15NontrivialUnpin<'a>(
            nontrivial: ::ctor::RvalueReference<'a, crate::NontrivialUnpin>,
        ) -> ::ctor::RvalueReference<'a, crate::NontrivialUnpin>;
        #[link_name = "_Z27TakesByConstRvalueReferenceOK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z27TakesByConstRvalueReferenceOK10Nontrivial<'a>(
            nontrivial: ::ctor::ConstRvalueReference<'a, crate::Nontrivial>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::Nontrivial>;
        #[link_name = "_Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z32TakesUnpinByConstRvalueReferenceOK15NontrivialUnpin<'a>(
            nontrivial: ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::NontrivialUnpin>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueC1ERKS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialByValue>,
            other: &'b crate::NontrivialByValue,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueC1EOS_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSERKS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: &'b crate::NontrivialByValue,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialByValue>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSEOS_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        ) -> ::core::pin::Pin<&'a mut crate::NontrivialByValue>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial<'a, 'other>(
            __return: &mut ::core::mem::MaybeUninit<crate::NontrivialByValue>,
            __this: ::core::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NonmovableC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NonmovableC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::Nonmovable>,
        );
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NonmovableD1Ev<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
        #[link_name = "_ZN10Nonmovable14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nonmovable14MemberFunctionEv<'a>(
            __this: ::core::pin::Pin<&'a mut crate::Nonmovable>,
        );
        pub(crate) fn __rust_thunk___Z24ReturnsNonmovableByValuev(
            __return: &mut ::core::mem::MaybeUninit<crate::Nonmovable>,
        );
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Nontrivial, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::core::mem::size_of::<crate::NontrivialInline>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::NontrivialInline>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialInline, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::core::mem::size_of::<crate::NontrivialMembers>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::NontrivialMembers>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);

const _: () = assert!(::core::mem::size_of::<crate::NontrivialUnpin>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::NontrivialUnpin>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialUnpin, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::core::mem::size_of::<crate::NontrivialByValue>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::NontrivialByValue>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);
};

const _: () = assert!(::core::mem::size_of::<crate::Nonmovable>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::Nonmovable>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
};
