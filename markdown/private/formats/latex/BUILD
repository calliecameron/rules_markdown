load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("//markdown/private/support/shell:defs.bzl", "sh_source")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

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
        "header_template.tex",
        "before_template.tex",
        "template.tex",
        "latex_filter.lua",
    ],
    visibility = ["//visibility:private"],
)

pandoc_filter_py_test(
    name = "latex_filter_test",
    src = "latex_filter_test.py",
    filter = "latex_filter.lua",
)

sh_source(
    name = "write_xelatex",
    src = "write_xelatex.sh",
)

genrule(
    name = "xelatex_sh",
    srcs = ["//markdown/private/external:xelatex"],
    outs = ["xelatex.sh"],
    cmd = "$(rootpath write_xelatex.sh) $< >$@",
    executable = True,
    tools = ["write_xelatex.sh"],
)

sh_binary(
    name = "xelatex",
    srcs = [":xelatex_sh"],
)
