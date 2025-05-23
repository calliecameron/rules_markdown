"""Formatter setup."""

load("@aspect_rules_lint//format:defs.bzl", "format_multirun", "format_test")

visibility("//")

# This is in a bzl file so we can share the langauge definitions between the two
# targets - kwargs doesn't work in build files, but does in bzl.

_LANGUAGES = {
    "shell": "@aspect_rules_lint//format:shfmt",
    "python": "@aspect_rules_lint//format:ruff",
}

def format(name):
    format_multirun(
        name = name,
        visibility = ["//visibility:private"],
        **_LANGUAGES
    )

    format_test(
        name = name + "_test",
        no_sandbox = True,
        workspace = "WORKSPACE",
        **_LANGUAGES
    )
