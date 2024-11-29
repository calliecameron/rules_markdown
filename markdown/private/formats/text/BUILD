load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

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
