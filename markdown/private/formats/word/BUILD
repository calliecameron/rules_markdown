load("@crates//:defs.bzl", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//markdown/private/support/shell:defs.bzl", "sh_binary", "sh_test")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test", "script_py_test")

exports_files(
    [
        "reference.docx",
        "docx_filter.lua",
        "shunn_docx_filter.lua",
    ],
    visibility = ["//visibility:private"],
)

sh_binary(
    name = "unoconv",
    srcs = ["unoconv.sh"],
    args = ["$(rootpath //markdown/private/external:unoconv)"],
    data = ["//markdown/private/external:unoconv"],
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
        "$(rootpath //markdown/private/external:md2short)",
        "$(rootpath //markdown/private/external:pandoc)",
        "$(rootpath //markdown/private/external:zip)",
        "$(rootpath //markdown/private/external:unzip)",
    ],
    data = [
        "//markdown/private/external:md2short",
        "//markdown/private/external:pandoc",
        "//markdown/private/external:unzip",
        "//markdown/private/external:zip",
    ],
)

pandoc_filter_py_test(
    name = "docx_filter_test",
    src = "docx_filter_test.py",
    filter = "docx_filter.lua",
)

sh_test(
    name = "shunn_docx_filter_test",
    srcs = ["shunn_docx_filter_test.sh"],
    args = [
        "$(rootpath //markdown/private/external:pandoc)",
        "$(rootpath //markdown/private/external:strip_nondeterminism)",
        "$(rootpath //markdown/private/external:zipinfo)",
        "$(rootpath shunn_docx_filter.lua)",
        "$(rootpath reference.docx)",
    ],
    data = [
        "reference.docx",
        "shunn_docx_filter.lua",
        "//markdown/private/external:pandoc",
        "//markdown/private/external:strip_nondeterminism",
        "//markdown/private/external:zipinfo",
    ],
)
