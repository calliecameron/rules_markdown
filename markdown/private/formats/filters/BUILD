load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

exports_files(
    [
        "add_title.lua",
        "add_subject.lua",
        "cleanup_metadata.lua",
        "remove_paragraph_annotations.lua",
        "remove_collection_separators.lua",
        "remove_collection_separators_before_headers.lua",
    ],
    visibility = ["//markdown/private/formats:__subpackages__"],
)

pandoc_filter_py_test(
    name = "add_title_test",
    src = "add_title_test.py",
    filter = "add_title.lua",
)

pandoc_filter_py_test(
    name = "add_subject_test",
    src = "add_subject_test.py",
    filter = "add_subject.lua",
)

pandoc_filter_py_test(
    name = "cleanup_metadata_test",
    src = "cleanup_metadata_test.py",
    filter = "cleanup_metadata.lua",
)

pandoc_filter_py_test(
    name = "remove_paragraph_annotations_test",
    src = "remove_paragraph_annotations_test.py",
    filter = "remove_paragraph_annotations.lua",
)

pandoc_filter_py_test(
    name = "remove_collection_separators_test",
    src = "remove_collection_separators_test.py",
    filter = "remove_collection_separators.lua",
)

pandoc_filter_py_test(
    name = "remove_collection_separators_before_headers_test",
    src = "remove_collection_separators_before_headers_test.py",
    filter = "remove_collection_separators_before_headers.lua",
)
