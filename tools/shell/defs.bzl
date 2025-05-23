"""Shell rules."""

load("@aspect_rules_lint//lint:lint_test.bzl", "lint_test")
load("@aspect_rules_lint//lint:shellcheck.bzl", "lint_shellcheck_aspect")

visibility("//...")

_shellcheck = lint_shellcheck_aspect(
    binary = "@multitool//tools/shellcheck",
    config = Label("//:.shellcheckrc"),
)

_shellcheck_test = lint_test(aspect = _shellcheck)

def sh_library(name, srcs = [], **kwargs):
    native.sh_library(
        name = name,
        srcs = srcs,
        **kwargs
    )
    _sh_lint(
        name = name,
        target = name,
    )

def sh_binary(name, srcs = [], **kwargs):
    native.sh_binary(
        name = name,
        srcs = srcs,
        **kwargs
    )
    _sh_lint(
        name = name,
        target = name,
    )

def sh_test(name, srcs = [], **kwargs):
    native.sh_test(
        name = name,
        srcs = srcs,
        **kwargs
    )
    _sh_lint(
        name = name,
        target = name,
    )

def sh_source(name, src, visibility = None):
    native.exports_files(
        [src],
        visibility = visibility or ["//visibility:private"],
    )
    native.sh_library(
        name = name + "_lint_lib",
        srcs = [src],
        visibility = ["//visibility:private"],
    )
    _sh_lint(
        name = name,
        target = name + "_lint_lib",
    )

def _sh_lint(name, target):
    _shellcheck_test(
        name = name + "_shellcheck_test",
        srcs = [target],
    )
