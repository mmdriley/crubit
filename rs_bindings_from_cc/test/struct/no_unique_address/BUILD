"""End-to-end test of no_unique_address."""

load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "no_unique_address",
    hdrs = ["no_unique_address.h"],
)

rust_test(
    name = "no_unique_address_test",
    srcs = ["no_unique_address_test.rs"],
    cc_deps = [":no_unique_address"],
    deps = ["//support:ctor"],
)
