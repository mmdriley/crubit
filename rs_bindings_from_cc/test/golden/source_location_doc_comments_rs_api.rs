// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:source_location_doc_comments_cc
// Features: experimental, supported

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls)]
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

#[derive(Clone, Copy)]
#[repr(C)]
pub struct StructFromMacro {
    /// A comment immediate above the macro expansion.
    pub val: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("StructFromMacro"),
    crate::StructFromMacro
);

// Error while generating bindings for item 'StructFromMacro::StructFromMacro':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'StructFromMacro::StructFromMacro':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'StructFromMacro::StructFromMacro':
// Parameter #0 is not supported: Unsupported type 'StructFromMacro &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'StructFromMacro::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'StructFromMacro::operator=':
// Parameter #0 is not supported: Unsupported type 'StructFromMacro &&': Unsupported type: && without lifetime

// A comment on a field of macro-generated struct.

#[::ctor::recursively_pinned]
#[repr(C)]
pub struct SomeStruct {
    /// A comment on a field of macro-generated struct.
    pub some_field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct"), crate::SomeStruct);

// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'SomeStruct::SomeStruct':
// Parameter #0 is not supported: Unsupported type 'SomeStruct &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'SomeStruct::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'SomeStruct::operator=':
// Parameter #0 is not supported: Unsupported type 'SomeStruct &&': Unsupported type: && without lifetime

// A doc comment on SomeStruct3 immediately above the macro expansion.

/// A comment on a macro-generated struct.
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct SomeStruct3 {
    __non_field_data: [::core::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("SomeStruct3"), crate::SomeStruct3);

// Error while generating bindings for item 'SomeStruct3::SomeStruct3':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'SomeStruct3::SomeStruct3':
// Unsafe constructors (e.g. with no elided or explicit lifetimes) are intentionally not supported

// Error while generating bindings for item 'SomeStruct3::SomeStruct3':
// Parameter #0 is not supported: Unsupported type 'SomeStruct3 &&': Unsupported type: && without lifetime

// Error while generating bindings for item 'SomeStruct3::operator=':
// `self` has no lifetime. Use lifetime annotations or `#pragma clang lifetime_elision` to create bindings for this function.

// Error while generating bindings for item 'SomeStruct3::operator=':
// Parameter #0 is not supported: Unsupported type 'SomeStruct3 &&': Unsupported type: && without lifetime

// THIRD_PARTY_CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_SOURCE_LOCATION_DOC_COMMENTS_H_

const _: () = assert!(::core::mem::size_of::<Option<&i32>>() == ::core::mem::size_of::<&i32>());

const _: () = assert!(::core::mem::size_of::<crate::StructFromMacro>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::StructFromMacro>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::StructFromMacro: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::StructFromMacro: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::StructFromMacro: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::StructFromMacro, val) == 0);

const _: () = assert!(::core::mem::size_of::<crate::SomeStruct>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::SomeStruct>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::SomeStruct, some_field) == 0);

const _: () = assert!(::core::mem::size_of::<crate::SomeStruct3>() == 1);
const _: () = assert!(::core::mem::align_of::<crate::SomeStruct3>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct3: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::SomeStruct3: Drop);
};
