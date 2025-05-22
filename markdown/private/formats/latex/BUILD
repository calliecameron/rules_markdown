load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")
load("//tools/lua:defs.bzl", "lua_source")
load("//tools/shell:defs.bzl", "sh_source")

exports_files(
    [
        "header_template.tex",
        "before_template.tex",
        "template.tex",
    ],
    visibility = ["//visibility:private"],
)

lua_source(
    name = "latex_filter",
    src = "latex_filter.lua",
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
    srcs = ["//tools/external:xelatex"],
    outs = ["xelatex.sh"],
    cmd = "$(execpath write_xelatex.sh) $< >$@",
    executable = True,
    tools = ["write_xelatex.sh"],
)

sh_binary(
    name = "xelatex",
    srcs = [":xelatex_sh"],
)
