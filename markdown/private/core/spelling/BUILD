load("//markdown/private/utils:defs.bzl", "pandoc_filter_py_test", "script_py_test")
load("//tools/lua:defs.bzl", "lua_source")
load("//tools/shell:defs.bzl", "sh_binary")

exports_files(
    [
        "spellcheck_input_template.md",
    ],
    visibility = ["//markdown/private/core:__pkg__"],
)

lua_source(
    name = "spellcheck_filter",
    src = "spellcheck_filter.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "spellcheck_filter_test",
    src = "spellcheck_filter_test.py",
    filter = "spellcheck_filter.lua",
)

lua_source(
    name = "spellcheck_cleanup",
    src = "spellcheck_cleanup.lua",
    visibility = ["//markdown/private/core:__pkg__"],
)

pandoc_filter_py_test(
    name = "spellcheck_cleanup_test",
    src = "spellcheck_cleanup_test.py",
    filter = "spellcheck_cleanup.lua",
)

sh_binary(
    name = "spellcheck",
    srcs = ["spellcheck.sh"],
    visibility = ["//markdown/private/core:__pkg__"],
)

script_py_test(
    name = "spellcheck_test",
    src = "spellcheck_test.py",
    args = [
        "$(rootpath //tools/external:hunspell)",
        "$(rootpath //tools/external:locale_archive)",
        "$(rootpaths //tools/external:hunspell_dicts)",
    ],
    data = [
        "//tools/external:hunspell",
        "//tools/external:hunspell_dicts",
        "//tools/external:locale_archive",
    ],
    script = ":spellcheck",
)

sh_binary(
    name = "gen_dictionary",
    srcs = ["gen_dictionary.sh"],
    visibility = ["//markdown/private/core:__pkg__"],
)

script_py_test(
    name = "gen_dictionary_test",
    src = "gen_dictionary_test.py",
    script = ":gen_dictionary",
)
