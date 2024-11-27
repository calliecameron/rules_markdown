load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@crates//:defs.bzl", "all_crate_deps")
load("@pip//:requirements.bzl", "requirement")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//markdown/private/support/python:defs.bzl", "py_binary")
load("//markdown/private/support/shell:defs.bzl", "sh_binary")
load("//markdown/private/utils:defs.bzl", "script_py_test")

bzl_library(
    name = "defs_bzl",
    srcs = ["defs.bzl"],
    visibility = [
        "//markdown/private:__pkg__",
        "//markdown/private/summary:__pkg__",
    ],
    deps = ["//markdown/private/core:defs_bzl"],
)

py_binary(
    name = "group_summary",
    srcs = ["group_summary.py"],
    type_stub_deps = [
        requirement("types-tabulate"),
    ],
    visibility = ["//visibility:public"],
    deps = [
        requirement("tabulate"),
        "//markdown/private/utils:metadata",
        "//markdown/private/utils:publications",
    ],
)

script_py_test(
    name = "group_summary_test",
    src = "group_summary_test.py",
    script = ":group_summary",
)

sh_binary(
    name = "write_group_summary_script",
    srcs = ["write_group_summary_script.sh"],
    visibility = ["//visibility:public"],
)

script_py_test(
    name = "write_group_summary_script_test",
    src = "write_group_summary_script_test.py",
    script = ":write_group_summary_script",
)

rust_binary(
    name = "group_publications",
    srcs = ["group_publications.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "group_publications_test",
    src = "group_publications_test.py",
    script = ":group_publications",
)

sh_binary(
    name = "write_group_publications_script",
    srcs = ["write_group_publications_script.sh"],
    visibility = ["//visibility:public"],
)

script_py_test(
    name = "write_group_publications_script_test",
    src = "write_group_publications_script_test.py",
    script = ":write_group_publications_script",
)
