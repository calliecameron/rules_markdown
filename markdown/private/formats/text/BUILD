load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

bzl_library(
    name = "defs_bzl",
    srcs = ["defs.bzl"],
    visibility = ["//markdown/private/formats:__pkg__"],
    deps = [
        "//markdown/private/formats:lib_bzl",
        "//markdown/private/formats:types_bzl",
    ],
)

exports_files(
    [
        "plain_markdown_filter.lua",
        "tumblr_markdown_filter.lua",
    ],
    visibility = ["//visibility:private"],
)

pandoc_filter_py_test(
    name = "plain_markdown_filter_test",
    src = "plain_markdown_filter_test.py",
    filter = "plain_markdown_filter.lua",
)

pandoc_filter_py_test(
    name = "tumblr_markdown_filter_test",
    src = "tumblr_markdown_filter_test.py",
    filter = "tumblr_markdown_filter.lua",
)
