load(":defs.bzl", "py_source")

py_source(
    name = "mypy_stub",
    src = "mypy_stub.py",
    visibility = ["//:__subpackages__"],
)

exports_files(
    [
        "python_version.bzl",
    ],
    visibility = ["//:__pkg__"],
)
