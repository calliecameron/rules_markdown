"""Public API of the module."""

load(
    "@rules_markdown//markdown/private:defs.bzl",
    _md_collection = "md_collection",
    _md_document = "md_document",
    _md_file = "md_file",
    _md_git_repo = "md_git_repo",
    _md_group = "md_group",
    _md_summary = "md_summary",
    _md_workspace = "md_workspace",
)
load(
    "//:cache.bzl",
    "cache_dir",
)
load(
    "//:versions.bzl",
    "version_file",
)

visibility("public")

def md_file(
        name,
        **kwargs):
    _md_file(
        name = name,
        version_file = version_file(native.package_name()),
        **kwargs
    )

def md_document(
        name,
        **kwargs):
    _md_document(
        name = name,
        version_file = version_file(native.package_name()),
        cache_dir = cache_dir(),
        **kwargs
    )

def md_collection(
        name,
        **kwargs):
    _md_collection(
        name = name,
        version_file = version_file(native.package_name()),
        cache_dir = cache_dir(),
        **kwargs
    )

md_group = _md_group
md_summary = _md_summary
md_git_repo = _md_git_repo
md_workspace = _md_workspace
