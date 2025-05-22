load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_test")
load("//markdown/private/utils:defs.bzl", "script_py_test")
load("//tools/shell:defs.bzl", "sh_binary")

exports_files(
    [
        "default_markdownlintrc",
    ],
    visibility = ["//visibility:public"],
)

sh_binary(
    name = "standard_lint",
    srcs = ["standard_lint.sh"],
    visibility = ["//markdown/private/core:__pkg__"],
)

rust_binary(
    name = "custom_lint",
    srcs = ["custom_lint.rs"],
    visibility = ["//markdown/private/core:__pkg__"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

rust_test(
    name = "custom_lint_rs_test",
    crate = ":custom_lint",
)

script_py_test(
    name = "custom_lint_test",
    src = "custom_lint_test.py",
    script = ":custom_lint",
)
