load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

exports_files(
    [
        "validate_ids.lua",
        "validate_quotes.lua",
        "include.lua",
        "paragraph_annotations.lua",
        "wordcount.lua",
        "header_auto_ids.lua",
        "poetry_lines.lua",
        "write_metadata.lua",
    ],
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "validate_ids_test",
    src = "validate_ids_test.py",
    filter = "validate_ids.lua",
)

pandoc_filter_py_test(
    name = "validate_quotes_test",
    src = "validate_quotes_test.py",
    filter = "validate_quotes.lua",
)

pandoc_filter_py_test(
    name = "include_test",
    src = "include_test.py",
    filter = "include.lua",
)

pandoc_filter_py_test(
    name = "paragraph_annotations_test",
    src = "paragraph_annotations_test.py",
    filter = "paragraph_annotations.lua",
)

pandoc_filter_py_test(
    name = "header_auto_ids_test",
    src = "header_auto_ids_test.py",
    filter = "header_auto_ids.lua",
)

pandoc_filter_py_test(
    name = "wordcount_test",
    src = "wordcount_test.py",
    filter = "wordcount.lua",
)

pandoc_filter_py_test(
    name = "poetry_lines_test",
    src = "poetry_lines_test.py",
    filter = "poetry_lines.lua",
)

pandoc_filter_py_test(
    name = "write_metadata_test",
    src = "write_metadata_test.py",
    filter = "write_metadata.lua",
)
