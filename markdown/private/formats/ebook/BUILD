load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//markdown/private/support/shell:defs.bzl", "sh_binary")

bzl_library(
    name = "defs_bzl",
    srcs = ["defs.bzl"],
    visibility = ["//markdown/private/formats:__pkg__"],
    deps = [
        "//markdown/private/core:defs_bzl",
        "//markdown/private/formats:lib_bzl",
    ],
)

exports_files(
    [
        "epub.css",
    ],
    visibility = ["//visibility:private"],
)

sh_binary(
    name = "ebook_convert",
    srcs = ["ebook_convert.sh"],
    args = ["$(rootpath //markdown/private/external:ebook_convert)"],
    data = ["//markdown/private/external:ebook_convert"],
    visibility = ["//markdown/private/formats:__pkg__"],
)
