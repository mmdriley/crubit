add_executable(rs_bindings_from_cc_impl
    ast_consumer.cc
    ast_convert.cc
    cmdline.cc
    frontend_action.cc
    importer.cc
    importers/function.cc
    importers/function_template.cc
    importers/cxx_record.cc
    importers/typedef_name.cc
    importers/enum.cc
    importers/namespace.cc
    importers/class_template.cc
    ir_from_cc.cc
    ir.cc
    rs_bindings_from_cc.cc
    src_code_gen.cc
    ../common/file_io.cc
)


target_link_libraries(rs_bindings_from_cc_impl
    # local
    ffi_types
    lifetime_annotations

    # abseil
    absl::flags_parse
    absl::status
    absl::statusor

    # llvm
    # LLVMSupport

    # clang
    clangTooling
)


target_link_libraries(rs_bindings_from_cc_impl
    ${CMAKE_SOURCE_DIR}/crates/target/debug/libsrc_code_gen.a
    dl
)



# other source files
add_library(json_from_cc
    json_from_cc.cc
)

# won't convert, header-only:
# cc_library(name = "bazel_types",
# cc_library(name = "decl_importer",
