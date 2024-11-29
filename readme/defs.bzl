"""Macros for use in the readme."""

load("@markdown//:defs.bzl", "md_file")
load("//markdown/private/utils:defs.bzl", "extend_file")

visibility("private")

def wrap_file(name, src, language):
    extend_file(
        name = name + "_md",
        src = src,
        out = name + ".md",
        append_lines = ["```"],
        prepend_lines = ["```" + language],
    )

    md_file(
        name = name,
        src = name + "_md",
        repo_override = "reproducible",
        version_override = "reproducible",
    )
