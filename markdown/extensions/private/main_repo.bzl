"""Repository rule for the main markdown repo."""

visibility("//markdown/extensions")

def _defs(repository_ctx):
    repository_ctx.file(
        "defs.bzl",
        content = repository_ctx.read(repository_ctx.attr._defs_bzl, watch = "no"),
        executable = False,
    )

def _versions(repository_ctx):
    bzl_entries = []
    for package, repo in sorted(repository_ctx.attr.package_repos.items()):
        bzl_entries.append("    \"%s\": \"@markdown//:%s_version.json\"," % (package, repo))

    repository_ctx.file(
        "versions.bzl",
        content = """visibility("private")

_VERSION_FILES = {
%s
}

def version_file(package):
    return _VERSION_FILES.get(package, None)
""" % "\n".join(bzl_entries),
        executable = False,
    )

    build_entries = []

    for repo in sorted(repository_ctx.attr.repos):
        build_entries.append("""alias(
    name = "%s_version.json",
    actual = "@%s//:version.json",
    visibility = ["//visibility:public"],
)""" % (repo, repo))

    return build_entries

def _cache(repository_ctx):
    result = repository_ctx.execute(
        ["mkdir", "-p", "cache"],
    )

    if result.return_code != 0:
        fail("Failed to create cache dir: %s" % result.stderr)

    result = repository_ctx.execute(
        ["readlink", "-f", "cache"],
    )

    if result.return_code != 0:
        fail("Failed to get path of cache dir: %s" % result.stderr)

    cache_dir = result.stdout.strip("\n")

    repository_ctx.file(
        "cache.bzl",
        content = """visibility("private")

def cache_dir():
    return \"%s\"
""" % cache_dir,
        executable = False,
    )

def _build(repository_ctx, build_entries):
    repository_ctx.file(
        "BUILD",
        content = """exports_files(
    [
        "defs.bzl",
    ],
    visibility = ["//visibility:public"],
)

""" + "\n\n".join(build_entries) + "\n",
        executable = False,
    )

def _main_repo_impl(repository_ctx):
    _defs(repository_ctx)
    build_entries = _versions(repository_ctx)
    _cache(repository_ctx)
    _build(repository_ctx, build_entries)

main_repo = repository_rule(
    implementation = _main_repo_impl,
    local = True,
    attrs = {
        "package_repos": attr.string_dict(mandatory = True),
        "repos": attr.string_list(mandatory = True),
        "_defs_bzl": attr.label(
            default = "//markdown/extensions/private:main_repo_defs.bzl",
        ),
    },
)
