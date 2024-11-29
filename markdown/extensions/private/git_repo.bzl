"""Repository rule for getting the version of a git repo."""

visibility("//markdown/extensions")

def _git_repo_impl(repository_ctx):
    files_to_watch = _git_internal_files(repository_ctx) + _git_repo_files(repository_ctx)
    for file in files_to_watch:
        path = repository_ctx.path(file)
        if path.is_dir:
            path.readdir(watch = "yes")
        else:
            repository_ctx.watch(path)

    result = repository_ctx.execute(
        [
            repository_ctx.attr._git_repo_version,
            repository_ctx.attr.root,
        ],
    )

    if result.return_code != 0:
        fail("Failed to get git repo version for repo %s: %s" %
             (repository_ctx.attr.root, result.stderr))

    repository_ctx.file(
        "version.json",
        content = json.encode_indent(
            {
                "version": result.stdout.strip("\n"),
                "repo": repository_ctx.attr.root,
            },
            indent = "    ",
        ) + "\n",
        executable = False,
    )

    repository_ctx.file(
        "BUILD",
        content = """
exports_files(
    ["version.json"],
    visibility = ["//visibility:public"],
)
        """,
        executable = False,
    )

git_repo = repository_rule(
    implementation = _git_repo_impl,
    local = True,
    attrs = {
        "root": attr.string(mandatory = True),
        "_git_repo_version": attr.label(
            default = "//markdown/private/utils:git_repo_version",
            executable = True,
            cfg = "exec",
        ),
    },
)

def _git_repo_files(repository_ctx):
    result = repository_ctx.execute(
        [
            "find",
            repository_ctx.attr.root,
            "-type",
            "f,d",
            "-name",
            ".git",
            "-prune",
            "-o",
            "-type",
            "f,d",
            "-print",
        ],
    )

    if result.return_code != 0:
        fail("Failed to search for git repo files in repo %s: %s" %
             (repository_ctx.attr.root, result.stderr))

    return result.stdout.strip("\n").split("\n")

def _git_internal_files(repository_ctx):
    root = repository_ctx.path(repository_ctx.attr.root)
    head = root.get_child(".git/HEAD")
    refs = root.get_child(".git/refs")

    if not head.exists or not refs.exists:
        fail("Path %s is not a git repo" % repository_ctx.attr.root)

    result = repository_ctx.execute(
        [
            "find",
            str(refs),
            "-type",
            "f,d",
        ],
    )

    if result.return_code != 0:
        fail("Failed to search for git internal files in repo %s: %s" %
             (repository_ctx.attr.root, result.stderr))

    files = result.stdout.strip("\n").split("\n")

    return [str(head)] + files
