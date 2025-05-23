"""Main module extension."""

load("//markdown/extensions/private:git_repo.bzl", "git_repo")
load("//markdown/extensions/private:main_repo.bzl", "main_repo")

visibility("private")

def _markdown_impl(module_ctx):
    root = module_ctx.path(Label("@@//:WORKSPACE")).dirname

    result = module_ctx.execute(
        [
            str(module_ctx.path(Label("//markdown/extensions/private:find_git_repos.py"))),
            str(root),
        ],
    )

    if result.return_code != 0:
        fail("Failed to find git repos: " + result.stderr)

    data = json.decode(result.stdout)

    for dir in data["dirs"]:
        root.get_child(dir).readdir(watch = "yes")

    for name, dir in sorted(data["repos"].items()):
        git_repo(
            name = name,
            root = str(root.get_child(dir)),
        )

    main_repo(
        name = "markdown",
        package_repos = data["packages"],
        repos = data["repos"].keys(),
    )

    return module_ctx.extension_metadata(reproducible = True)

markdown = module_extension(
    implementation = _markdown_impl,
)
