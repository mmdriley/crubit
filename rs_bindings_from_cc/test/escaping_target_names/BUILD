load("@rules_rust//rust:defs.bzl", "rust_test")
load("//rs_bindings_from_cc/test:test_bindings.bzl", "crubit_test_cc_library")

package(default_applicable_licenses = ["//third_party/crubit:license"])

licenses(["notice"])

crubit_test_cc_library(
    name = "has-dashes-in-name",
    hdrs = ["has-dashes-in-name.h"],
)

crubit_test_cc_library(
    name = "uses-target-with-dashes",
    srcs = ["uses-target-with-dashes.cc"],
    hdrs = ["uses-target-with-dashes.h"],
    deps = [":has-dashes-in-name"],
)

rust_test(
    name = "escaping_target_names",
    srcs = ["escaping_target_names.rs"],
    cc_deps = [
        ":has-dashes-in-name",
        ":uses-target-with-dashes",
    ],
)
