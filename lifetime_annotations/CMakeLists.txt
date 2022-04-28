add_library(lifetime_annotations
    lifetime.cc
    function_lifetimes.cc
    type_lifetimes.cc
    lifetime_annotations.cc
    lifetime_substitutions.cc
    lifetime_symbol_table.cc
    pointee_type.cc
)

# dependencies:
#        "@absl//strings:str_format",
#        "@absl//strings",
#        "@llvm///clang:ast",
#        "@llvm///clang:basic",
#        "@llvm///clang:frontend",
#        "@llvm///clang:lex",
#        "@llvm///llvm:Support",
