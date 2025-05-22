load("//tools/python:defs.bzl", "py_source")
load("//tools/shell:defs.bzl", "sh_test")

py_source(
    name = "find_git_repos",
    src = "find_git_repos",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "find_git_repos_test",
    srcs = ["find_git_repos_test.sh"],
    args = ["$(rootpath find_git_repos)"],
    data = ["find_git_repos"],
)
