load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//markdown/private/utils:defs.bzl", "script_py_test")

exports_files(
    [
        "collection_header.tex",
        "collection_before.tex",
    ],
    visibility = ["//visibility:public"],
)

rust_binary(
    name = "gen_collection_src",
    srcs = ["gen_collection_src.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "gen_collection_src_test",
    src = "gen_collection_src_test.py",
    script = ":gen_collection_src",
)
