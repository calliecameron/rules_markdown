load("//markdown/private/support/lua:defs.bzl", "lua_source")
load("//markdown/private/support/shell:defs.bzl", "sh_source")
load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test")

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
    srcs = ["//markdown/private/external:xelatex"],
    outs = ["xelatex.sh"],
    cmd = "$(execpath write_xelatex.sh) $< >$@",
    executable = True,
    tools = ["write_xelatex.sh"],
)

sh_binary(
    name = "xelatex",
    srcs = [":xelatex_sh"],
)
