"""End-to-end example of using type aliases to fully-instantiated templates."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "class_template_definition",
    hdrs = ["class_template_definition.h"],
)

crubit_test_cc_library(
    name = "class_template_instantiation1",
    hdrs = ["class_template_instantiation1.h"],
    deps = [":class_template_definition"],
)

crubit_test_cc_library(
    name = "class_template_instantiation2",
    hdrs = ["class_template_instantiation2.h"],
    deps = [":class_template_definition"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [
        ":class_template_instantiation1",
        ":class_template_instantiation2",
    ],
    deps = [
        "//support:forward_declare",
    ],
)
