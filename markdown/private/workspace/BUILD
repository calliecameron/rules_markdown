load("//tools/shell:defs.bzl", "sh_source", "sh_test")

exports_files(
    [
        "default_bazeliskrc",
        "default_bazelrc",
    ],
    visibility = ["//visibility:public"],
)

sh_source(
    name = "new_package",
    src = "new_package.sh",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "new_package_test",
    srcs = ["new_package_test.sh"],
    args = ["$(rootpath new_package.sh)"],
    data = ["new_package.sh"],
)

sh_source(
    name = "workspace_status",
    src = "workspace_status.sh",
    visibility = ["//visibility:public"],
)

sh_test(
    name = "workspace_status_test",
    srcs = ["workspace_status_test.sh"],
    args = [
        "$(rootpath workspace_status.sh)",
        "$(rootpath //markdown/private/utils:git_repo_version.py)",
    ],
    data = [
        "workspace_status.sh",
        "//markdown/private/utils:git_repo_version.py",
    ],
)
