load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")
load("//tools/lua:defs.bzl", "lua_source")

lua_source(
    name = "validate_ids",
    src = "validate_ids.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "validate_ids_test",
    src = "validate_ids_test.py",
    filter = "validate_ids.lua",
)

lua_source(
    name = "validate_quotes",
    src = "validate_quotes.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "validate_quotes_test",
    src = "validate_quotes_test.py",
    filter = "validate_quotes.lua",
)

lua_source(
    name = "include",
    src = "include.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "include_test",
    src = "include_test.py",
    filter = "include.lua",
)

lua_source(
    name = "paragraph_annotations",
    src = "paragraph_annotations.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "paragraph_annotations_test",
    src = "paragraph_annotations_test.py",
    filter = "paragraph_annotations.lua",
)

lua_source(
    name = "header_auto_ids",
    src = "header_auto_ids.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "header_auto_ids_test",
    src = "header_auto_ids_test.py",
    filter = "header_auto_ids.lua",
)

lua_source(
    name = "wordcount",
    src = "wordcount.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "wordcount_test",
    src = "wordcount_test.py",
    filter = "wordcount.lua",
)

lua_source(
    name = "poetry_lines",
    src = "poetry_lines.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "poetry_lines_test",
    src = "poetry_lines_test.py",
    filter = "poetry_lines.lua",
)

lua_source(
    name = "write_metadata",
    src = "write_metadata.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "write_metadata_test",
    src = "write_metadata_test.py",
    filter = "write_metadata.lua",
)
