"""Summary implementation macros."""

load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@rules_markdown//markdown/private/utils:defs.bzl", "required_files")

def md_summary_impl(contents, md_group):
    bzl_library(
        name = "contents_bzl",
        srcs = ["contents.bzl"],
        visibility = ["//visibility:private"],
    )

    required_files(
        name = "contents",
        copy = [
            (
                "@rules_markdown//markdown/private/summary:contents.build",
                "BUILD",
                "600",
            ),
            (
                "@rules_markdown//markdown/private/summary:refresh",
                "refresh",
                "700",
            ),
        ],
        create = [
            (
                "@rules_markdown//markdown/private/summary:contents.bzl",
                "contents.bzl",
                "600",
            ),
        ],
    )

    md_group(
        name = "contents",
        deps = contents,
    )
