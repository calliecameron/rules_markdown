load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test", "script_py_test")
load("//tools/lua:defs.bzl", "lua_source")
load("//tools/shell:defs.bzl", "sh_binary", "sh_test")

exports_files(
    [
        "reference.docx",
    ],
    visibility = ["//visibility:private"],
)

sh_binary(
    name = "unoconv",
    srcs = ["unoconv.sh"],
    args = ["$(rootpath //tools/external:unoconv)"],
    data = ["//tools/external:unoconv"],
)

rust_binary(
    name = "shunn_metadata",
    srcs = ["shunn_metadata.rs"],
    deps = [
        "//markdown/private/utils",
    ] + all_crate_deps(
        normal = True,
    ),
)

script_py_test(
    name = "shunn_metadata_test",
    src = "shunn_metadata_test.py",
    script = ":shunn_metadata",
)

sh_binary(
    name = "md2short",
    srcs = ["md2short.sh"],
    args = [
        "$(rootpath //tools/external:md2short)",
        "$(rootpath //tools/external:pandoc)",
        "$(rootpath //tools/external:zip)",
        "$(rootpath //tools/external:unzip)",
    ],
    data = [
        "//tools/external:md2short",
        "//tools/external:pandoc",
        "//tools/external:unzip",
        "//tools/external:zip",
    ],
)

lua_source(
    name = "docx_filter",
    src = "docx_filter.lua",
)

pandoc_filter_py_test(
    name = "docx_filter_test",
    src = "docx_filter_test.py",
    filter = "docx_filter.lua",
)

lua_source(
    name = "shunn_docx_filter",
    src = "shunn_docx_filter.lua",
)

sh_test(
    name = "shunn_docx_filter_test",
    srcs = ["shunn_docx_filter_test.sh"],
    args = [
        "$(rootpath //tools/external:pandoc)",
        "$(rootpath //tools/external:strip_nondeterminism)",
        "$(rootpath //tools/external:zipinfo)",
        "$(rootpath shunn_docx_filter.lua)",
        "$(rootpath reference.docx)",
    ],
    data = [
        "reference.docx",
        "shunn_docx_filter.lua",
        "//tools/external:pandoc",
        "//tools/external:strip_nondeterminism",
        "//tools/external:zipinfo",
    ],
)
