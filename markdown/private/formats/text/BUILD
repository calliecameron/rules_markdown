load("//markdown/private/support/lua:defs.bzl", "lua_source")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

lua_source(
    name = "plain_markdown_filter",
    src = "plain_markdown_filter.lua",
)

pandoc_filter_py_test(
    name = "plain_markdown_filter_test",
    src = "plain_markdown_filter_test.py",
    filter = "plain_markdown_filter.lua",
)

lua_source(
    name = "tumblr_markdown_filter",
    src = "tumblr_markdown_filter.lua",
)

pandoc_filter_py_test(
    name = "tumblr_markdown_filter_test",
    src = "tumblr_markdown_filter_test.py",
    filter = "tumblr_markdown_filter.lua",
)
