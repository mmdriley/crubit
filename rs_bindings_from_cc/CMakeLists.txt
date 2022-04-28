add_library(ir
    ir.cc
)

add_library(ast_convert
    ast_convert.cc
)

add_library(src_code_gen
    src_code_gen.cc
)

add_library(json_from_cc
    json_from_cc.cc
)

add_library(ir_from_cc
    ir_from_cc.cc
)

add_library(importer
    importer.cc
)

# todo:
# cc_binary(name = "rs_bindings_from_cc_impl",
# cc_library(name = "cmdline",
# cc_library(name = "decl_importer",
# cc_library(name = "frontend_action",
# cc_library(name = "ast_consumer",
# cc_library(name = "importer",

# won't convert:
# cc_library(name = "bazel_types",
#   header-only
