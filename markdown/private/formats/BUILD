load("//markdown/private/utils:defs.bzl", "script_py_test")
load("//tools/shell:defs.bzl", "sh_binary")

sh_binary(
    name = "write_open_script",
    srcs = ["write_open_script.sh"],
    visibility = ["//markdown/private/formats:__subpackages__"],
)

script_py_test(
    name = "write_open_script_test",
    src = "write_open_script_test.py",
    script = ":write_open_script",
)

sh_binary(
    name = "zip_cleaner",
    srcs = ["zip_cleaner.sh"],
    args = ["$(rootpath //tools/external:strip_nondeterminism)"],
    data = ["//tools/external:strip_nondeterminism"],
    visibility = ["//markdown/private/formats:__subpackages__"],
)

script_py_test(
    name = "zip_cleaner_test",
    src = "zip_cleaner_test.py",
    args = [
        "$(rootpath //tools/external:strip_nondeterminism)",
        "$(rootpath //tools/external:zipinfo)",
        "$(rootpath //tools/external:zip)",
    ],
    data = [
        "//tools/external:strip_nondeterminism",
        "//tools/external:zip",
        "//tools/external:zipinfo",
    ],
    script = ":zip_cleaner",
)

sh_binary(
    name = "write_save_script",
    srcs = ["write_save_script.sh"],
    visibility = ["//visibility:public"],
)

script_py_test(
    name = "write_save_script_test",
    src = "write_save_script_test.py",
    script = ":write_save_script",
)
