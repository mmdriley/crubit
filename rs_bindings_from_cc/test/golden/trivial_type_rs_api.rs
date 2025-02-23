// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:trivial_type_cc
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

pub mod ns {
    /// Implicitly defined special member functions are trivial on a struct with
    /// only trivial members.
    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct Trivial {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(forward_declare::symbol!("Trivial"), crate::ns::Trivial);

    impl Default for Trivial {
        #[inline(always)]
        fn default() -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1Ev(&mut tmp);
                tmp.assume_init()
            }
        }
    }

    impl<'b> From<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
            let mut tmp = ::core::mem::MaybeUninit::<Self>::zeroed();
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialC1EOS0_(&mut tmp, __param_0);
                tmp.assume_init()
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<&'b Self> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSERKS0_(self, __param_0);
            }
        }
    }

    impl<'b> ::ctor::UnpinAssign<::ctor::RvalueReference<'b, Self>> for Trivial {
        #[inline(always)]
        fn unpin_assign<'a>(&'a mut self, __param_0: ::ctor::RvalueReference<'b, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns7TrivialaSEOS0_(self, __param_0);
            }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn Unqualified<'a>(&'a mut self) {
            unsafe { crate::detail::__rust_thunk___ZN2ns7Trivial11UnqualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn ConstQualified<'a>(&'a self) {
            unsafe { crate::detail::__rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn LvalueRefQualified<'a>(&'a mut self) {
            unsafe { crate::detail::__rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn ConstLvalueRefQualified<'a>(&'a self) {
            unsafe {
                crate::detail::__rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv(self)
            }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn RvalueRefQualified<'a>(self: ::ctor::RvalueReference<'a, Self>) {
            unsafe { crate::detail::__rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv(self) }
        }
    }

    impl Trivial {
        #[inline(always)]
        pub fn ConstRvalueRefQualified<'a>(self: ::ctor::ConstRvalueReference<'a, Self>) {
            unsafe {
                crate::detail::__rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv(self)
            }
        }
    }

    /// This struct is trivial, and therefore trivially relocatable etc., but still
    /// not safe to pass by reference as it is not final.
    #[::ctor::recursively_pinned]
    #[repr(C)]
    pub struct TrivialNonfinal {
        pub trivial_field: i32,
    }
    forward_declare::unsafe_define!(
        forward_declare::symbol!("TrivialNonfinal"),
        crate::ns::TrivialNonfinal
    );

    impl ::ctor::CtorNew<()> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self>;
        #[inline(always)]
        fn ctor_new(args: ()) -> Self::CtorType {
            let () = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1Ev(
                            ::core::pin::Pin::into_inner_unchecked(dest),
                        );
                    },
                )
            }
        }
    }

    impl<'b> ::ctor::CtorNew<&'b Self> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: &'b Self) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_(
                            ::core::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(&'b Self,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
            let __param_0 = args;
            unsafe {
                ::ctor::FnCtor::new(
                    move |dest: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>| {
                        crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_(
                            ::core::pin::Pin::into_inner_unchecked(dest),
                            __param_0,
                        );
                    },
                )
            }
        }
    }
    impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for TrivialNonfinal {
        type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
        #[inline(always)]
        fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
            let (arg,) = args;
            <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
        }
    }

    impl<'b> ::ctor::Assign<&'b Self> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(self: ::core::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_(self, __param_0);
            }
        }
    }

    impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for TrivialNonfinal {
        #[inline(always)]
        fn assign<'a>(
            self: ::core::pin::Pin<&'a mut Self>,
            __param_0: ::ctor::RvalueReference<'b, Self>,
        ) {
            unsafe {
                crate::detail::__rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_(self, __param_0);
            }
        }
    }

    #[inline(always)]
    pub fn TakesByValue(mut trivial: crate::ns::Trivial) -> crate::ns::Trivial {
        unsafe {
            let mut __return = ::core::mem::MaybeUninit::<crate::ns::Trivial>::uninit();
            crate::detail::__rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
                &mut __return,
                &mut trivial,
            );
            __return.assume_init()
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByValue(
        trivial: impl ::ctor::Ctor<Output = crate::ns::TrivialNonfinal>,
    ) -> impl ::ctor::Ctor<Output = crate::ns::TrivialNonfinal> {
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::core::pin::Pin<
                    &mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
                >| {
                    crate::detail::__rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(::core::pin::Pin::into_inner_unchecked(dest),::core::pin::Pin::into_inner_unchecked(::ctor::emplace!(trivial)));
                },
            )
        }
    }

    #[inline(always)]
    pub fn TakesByReference<'a>(trivial: &'a mut crate::ns::Trivial) -> &'a mut crate::ns::Trivial {
        unsafe { crate::detail::__rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE(trivial) }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByReference<'a>(
        trivial: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
    ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByConstReference<'a>(trivial: &'a crate::ns::Trivial) -> &'a crate::ns::Trivial {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByConstReference<'a>(
        trivial: &'a crate::ns::TrivialNonfinal,
    ) -> &'a crate::ns::TrivialNonfinal {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByRvalueReference<'a>(
        trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>,
    ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE(trivial)
        }
    }

    #[inline(always)]
    pub fn TakesTrivialNonfinalByConstRvalueReference<'a>(
        trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>,
    ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal> {
        unsafe {
            crate::detail::__rust_thunk___ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE(trivial)
        }
    }
}

// namespace ns

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_TRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::Trivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns7TrivialaSERKS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: &'b crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        pub(crate) fn __rust_thunk___ZN2ns7TrivialaSEOS0_<'a, 'b>(
            __this: &'a mut crate::ns::Trivial,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::Trivial>,
        ) -> &'a mut crate::ns::Trivial;
        #[link_name = "_ZN2ns7Trivial11UnqualifiedEv"]
        pub(crate) fn __rust_thunk___ZN2ns7Trivial11UnqualifiedEv<'a>(
            __this: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZNK2ns7Trivial14ConstQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNK2ns7Trivial14ConstQualifiedEv<'a>(
            __this: &'a crate::ns::Trivial,
        );
        #[link_name = "_ZNR2ns7Trivial18LvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNR2ns7Trivial18LvalueRefQualifiedEv<'a>(
            __this: &'a mut crate::ns::Trivial,
        );
        #[link_name = "_ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKR2ns7Trivial23ConstLvalueRefQualifiedEv<'a>(
            __this: &'a crate::ns::Trivial,
        );
        #[link_name = "_ZNO2ns7Trivial18RvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNO2ns7Trivial18RvalueRefQualifiedEv<'a>(
            __this: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
        );
        #[link_name = "_ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv"]
        pub(crate) fn __rust_thunk___ZNKO2ns7Trivial23ConstRvalueRefQualifiedEv<'a>(
            __this: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1Ev<'a>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1ERKS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalC1EOS0_<'a, 'b>(
            __this: &'a mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        );
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSERKS0_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: &'b crate::ns::TrivialNonfinal,
        ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        pub(crate) fn __rust_thunk___ZN2ns15TrivialNonfinalaSEOS0_<'a, 'b>(
            __this: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
            __param_0: ::ctor::RvalueReference<'b, crate::ns::TrivialNonfinal>,
        ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        pub(crate) fn __rust_thunk___ZN2ns12TakesByValueENS_7TrivialE(
            __return: &mut ::core::mem::MaybeUninit<crate::ns::Trivial>,
            trivial: &mut crate::ns::Trivial,
        );
        pub(crate) fn __rust_thunk___ZN2ns27TakesTrivialNonfinalByValueENS_15TrivialNonfinalE(
            __return: &mut ::core::mem::MaybeUninit<crate::ns::TrivialNonfinal>,
            trivial: &mut crate::ns::TrivialNonfinal,
        );
        #[link_name = "_ZN2ns16TakesByReferenceERNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns16TakesByReferenceERNS_7TrivialE<'a>(
            trivial: &'a mut crate::ns::Trivial,
        ) -> &'a mut crate::ns::Trivial;
        #[link_name = "_ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns31TakesTrivialNonfinalByReferenceERNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>,
        ) -> ::core::pin::Pin<&'a mut crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns21TakesByConstReferenceERKNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns21TakesByConstReferenceERKNS_7TrivialE<'a>(
            trivial: &'a crate::ns::Trivial,
        ) -> &'a crate::ns::Trivial;
        #[link_name = "_ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns36TakesTrivialNonfinalByConstReferenceERKNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: &'a crate::ns::TrivialNonfinal,
        ) -> &'a crate::ns::TrivialNonfinal;
        #[link_name = "_ZN2ns22TakesByRvalueReferenceEONS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns22TakesByRvalueReferenceEONS_7TrivialE<'a>(
            trivial: ::ctor::RvalueReference<'a, crate::ns::Trivial>,
        ) -> ::ctor::RvalueReference<'a, crate::ns::Trivial>;
        #[link_name = "_ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns37TakesTrivialNonfinalByRvalueReferenceEONS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>,
        ) -> ::ctor::RvalueReference<'a, crate::ns::TrivialNonfinal>;
        #[link_name = "_ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE"]
        pub(crate) fn __rust_thunk___ZN2ns27TakesByConstRvalueReferenceEOKNS_7TrivialE<'a>(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::Trivial>;
        #[link_name = "_ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE"]
        pub(crate) fn __rust_thunk___ZN2ns42TakesTrivialNonfinalByConstRvalueReferenceEOKNS_15TrivialNonfinalE<
            'a,
        >(
            trivial: ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>,
        ) -> ::ctor::ConstRvalueReference<'a, crate::ns::TrivialNonfinal>;
    }
}

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::ns::Trivial>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::ns::Trivial>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ns::Trivial: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::Trivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::Trivial, trivial_field) == 0);
const _: () = assert!(::core::mem::size_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::ns::TrivialNonfinal>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ns::TrivialNonfinal: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::ns::TrivialNonfinal, trivial_field) == 0);
