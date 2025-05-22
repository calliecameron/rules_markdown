"""Shell rules."""

load("@aspect_rules_lint//lint:lint_test.bzl", "lint_test")
load("@aspect_rules_lint//lint:shellcheck.bzl", "lint_shellcheck_aspect")
load("@bazel_skylib//rules:native_binary.bzl", "native_test")

_shellcheck = lint_shellcheck_aspect(
    binary = "@multitool//tools/shellcheck",
    config = Label("//:.shellcheckrc"),
)

_shellcheck_test = lint_test(aspect = _shellcheck)

visibility("//...")

def sh_library(name, srcs = [], **kwargs):
    native.sh_library(
        name = name,
        srcs = srcs,
        **kwargs
    )
    _sh_lint(
        name = name,
        srcs = srcs,
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
        srcs = srcs,
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
        srcs = srcs,
        target = name,
    )

def sh_source(name, src, visibility = None):
    native.exports_files(
        [src],
        visibility = visibility or ["//visibility:private"],
    )
    native.sh_library(
        name = name + "_lib_for_shellcheck",
        srcs = [src],
        visibility = ["//visibility:private"],
    )
    _sh_lint(
        name = name,
        srcs = [src],
        target = name + "_lib_for_shellcheck",
    )

def _sh_lint(name, srcs, target):
    _shellcheck_test(
        name = name + "_shellcheck_test",
        srcs = [target],
    )

    if not srcs:
        return

    native_test(
        name = name + "_shfmt_test",
        src = "@aspect_rules_lint//format:shfmt",
        out = name + "_shfmt",
        args = [
            "-l",
            "-d",
            "-i",
            "4",
        ] + ["$(location %s)" % src for src in srcs],
        data = srcs,
    )
