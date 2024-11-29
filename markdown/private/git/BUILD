load("//markdown/private/support/shell:defs.bzl", "sh_binary", "sh_source", "sh_test")

exports_files(
    [
        "default_gitattributes",
        "default_gitconfig",
        "default_gitignore",
    ],
    visibility = ["//visibility:public"],
)

sh_source(
    name = "precommit_template",
    src = "precommit_template",
    visibility = ["//visibility:public"],
)

sh_source(
    name = "run_tests",
    src = "run_tests",
    visibility = ["//visibility:public"],
)

sh_binary(
    name = "git_update_extra",
    srcs = ["git_update_extra.sh"],
    visibility = ["//visibility:public"],
)

sh_test(
    name = "git_update_extra_test",
    srcs = ["git_update_extra_test.sh"],
    args = [
        "$(rootpath :git_update_extra)",
    ],
    data = [
        ":git_update_extra",
    ],
)

sh_source(
    name = "git_test_extra",
    src = "git_test_extra.sh",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "git_test_extra_test",
    srcs = ["git_test_extra_test.sh"],
    args = [
        "$(rootpath git_test_extra.sh)",
    ],
    data = [
        "git_test_extra.sh",
    ],
)
