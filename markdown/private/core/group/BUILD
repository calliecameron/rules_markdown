load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//markdown/private/utils:defs.bzl", "script_py_test")

rust_binary(
    name = "combine_deps_metadata",
    srcs = ["combine_deps_metadata.rs"],
    visibility = ["//markdown/private/core:__pkg__"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "combine_deps_metadata_test",
    src = "combine_deps_metadata_test.py",
    script = ":combine_deps_metadata",
)
