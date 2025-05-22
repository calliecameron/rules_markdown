load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//markdown/private/utils:defs.bzl", "script_py_test")
load("//tools/shell:defs.bzl", "sh_binary")

rust_binary(
    name = "group_summary",
    srcs = ["group_summary.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
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
