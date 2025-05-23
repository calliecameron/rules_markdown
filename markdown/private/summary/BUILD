load("//tools/shell:defs.bzl", "sh_source")

exports_files(
    [
        "contents.build",
        "contents.bzl",
    ],
    visibility = ["//visibility:public"],
)

sh_source(
    name = "refresh",
    src = "refresh.sh",
    visibility = ["//visibility:public"],
)

sh_source(
    name = "summarise_publications",
    src = "summarise_publications.sh",
    visibility = ["//visibility:public"],
)

sh_source(
    name = "summarise_contents",
    src = "summarise_contents.sh",
    visibility = ["//visibility:public"],
)
