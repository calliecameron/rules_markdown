load("//markdown/private/support/lua:defs.bzl", "lua_source")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

lua_source(
    name = "add_title",
    src = "add_title.lua",
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "add_title_test",
    src = "add_title_test.py",
    filter = "add_title.lua",
)

lua_source(
    name = "add_subject",
    src = "add_subject.lua",
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "add_subject_test",
    src = "add_subject_test.py",
    filter = "add_subject.lua",
)

lua_source(
    name = "cleanup_metadata",
    src = "cleanup_metadata.lua",
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "cleanup_metadata_test",
    src = "cleanup_metadata_test.py",
    filter = "cleanup_metadata.lua",
)

lua_source(
    name = "remove_paragraph_annotations",
    src = "remove_paragraph_annotations.lua",
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "remove_paragraph_annotations_test",
    src = "remove_paragraph_annotations_test.py",
    filter = "remove_paragraph_annotations.lua",
)

lua_source(
    name = "remove_collection_separators",
    src = "remove_collection_separators.lua",
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "remove_collection_separators_test",
    src = "remove_collection_separators_test.py",
    filter = "remove_collection_separators.lua",
)

lua_source(
    name = "remove_collection_separators_before_headers",
    src = "remove_collection_separators_before_headers.lua",
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "remove_collection_separators_before_headers_test",
    src = "remove_collection_separators_before_headers_test.py",
    filter = "remove_collection_separators_before_headers.lua",
)
