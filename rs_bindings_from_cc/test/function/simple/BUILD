"""End-to-end example of using simple functions."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "simple_functions",
    srcs = ["simple_functions.cc"],
    hdrs = ["simple_functions.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":simple_functions"],
)
