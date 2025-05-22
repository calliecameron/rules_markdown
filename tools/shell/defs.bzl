"""Shell rules."""

load("@bazel_skylib//rules:native_binary.bzl", "native_test")

visibility("//...")

def sh_library(name, **kwargs):
    _sh_lint(
        name = name,
        **kwargs
    )
    native.sh_library(
        name = name,
        **kwargs
    )

def sh_binary(name, **kwargs):
    _sh_lint(
        name = name,
        **kwargs
    )
    native.sh_binary(
        name = name,
        **kwargs
    )

def sh_test(name, **kwargs):
    _sh_lint(
        name = name,
        **kwargs
    )
    native.sh_test(
        name = name,
        **kwargs
    )

def sh_source(name, src, visibility = None):
    native.exports_files(
        [src],
        visibility = visibility or ["//visibility:private"],
    )
    _sh_lint(
        name = name,
        srcs = [src],
    )

def _sh_lint(name, **kwargs):
    srcs = kwargs.get("srcs", [])

    if not srcs:
        return

    native.sh_test(
        name = name + "_shellcheck_test",
        srcs = ["//tools/shell:shellcheck_test.sh"],
        args = [
            "$(rootpath //tools/external:shellcheck)",
        ] + ["$(location %s)" % src for src in srcs],
        data = [
            "//tools/external:shellcheck",
        ] + srcs,
    )

    native_test(
        name = name + "_shfmt_test",
        src = "//tools/external:shfmt",
        out = name + "_shfmt",
        args = [
            "-l",
            "-d",
            "-i",
            "4",
        ] + ["$(location %s)" % src for src in srcs],
        data = srcs,
    )
