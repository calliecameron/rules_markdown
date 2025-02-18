# rules_markdown

Bazel rules for markdown. Wraps pandoc and other tools for validation and
conversion to other formats.

Output formats include:

-   `pdf`
-   `epub`, `mobi`
-   `docx`, `odt`, `doc`
-   `docx` in [Shunn standard manuscript
    format](https://www.shunn.net/format/story/)

## Prerequisites

-   System packages:

    ``` shell
    # Ubuntu 22.04
    sudo apt-get install catdoc g++ gcc git
    ```

-   [Nix](https://nixos.org/)

-   Bazel via [Bazelisk](https://github.com/bazelbuild/bazelisk)

## Setup

Create the following files in your workspace:

`.bazeliskrc`:

``` text
USE_BAZEL_VERSION=7.4.1
```

`.bazelrc`:

``` text
common --experimental_sandbox_async_tree_delete_idle_threads=0
common --experimental_isolated_extension_usages
build "--workspace_status_command=/bin/bash -c 'if [ -x ./.markdown_workspace/workspace_status ] && [ -x ./.markdown_workspace/git_repo_version ]; then ./.markdown_workspace/workspace_status ./.markdown_workspace/git_repo_version; fi'"
build --nobuild_runfile_links --sandbox_default_allow_network=false
test --nobuild_runfile_links --build_tests_only
```

`WORKSPACE`: empty file

`MODULE.bazel`:

``` starlark
"""Example MODULE.bazel."""

module(
    name = "my_module",
    version = "0.0.0",
)

bazel_dep(
    name = "rules_markdown",
    version = "0.18.2",
)
archive_override(
    module_name = "rules_markdown",
    # Set these from the corresponding tag on github
    integrity = "<from github>",
    strip_prefix = "<from github>",
    urls = "<from github>",
)

markdown = use_extension("@rules_markdown//markdown/extensions:markdown.bzl", "markdown")
use_repo(markdown, "markdown")
```

`BUILD`:

``` starlark
load("@markdown//:defs.bzl", "md_git_repo", "md_workspace")

md_workspace()

# Only if the workspace is at the root of a git repo.
md_git_repo()
```

Initialise with:

``` shell
bazel run :workspace_update
bazel run :contents_update
# Only if using md_git_repo
bazel run :git_update
```

And verify with:

``` shell
bazel test :all
```

## Usage

``` starlark
load("@markdown//:defs.bzl", "md_document", "md_file")

# md_file validates the file (spellchecking, lint, etc.) and handles
# dependencies (see 'Includes', below); run `bazel test :foo` to validate. See
# the included bzl file for full documentation.
md_file(
    name = "foo",
    # 'src' is optional; defaults to foo.md based on 'name'
)

# md_document does everything md_file does, plus conversion to other formats;
# `bazel build <extension>` e.g. `bazel build :pdf` or `bazel build :epub` to
# convert. `bazel run <extension>` to open the output in the default viewer. All
# md_file arguments are also valid for md_document.
md_document(
    name = "bar",
    # 'src' is optional; defaults to bar.md based on 'name'
)
```

To initialise a subdirectory with a default BUILD file, run `bazel run //:new`
in that directory.

Markdown files use [pandoc
markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), and can have the
standard pandoc metadata – title, author, date – in a yaml front matter block.

See files under ‘tests’ for full examples.

### Spellchecking

``` starlark
load("@markdown//:defs.bzl", "md_document")

# Uses custom dictionary foo.dic based on 'name', if it exists.
md_document(
    name = "foo",
    # Other custom dictionaries can be added with `extra_dictionaries`
    extra_dictionaries = ["extra.dic"],
)
```

A custom dictionary is a file with one word per line, containing words that
should be considered correct.

### Includes

Include a file in another with:

``` markdown
!include $LABEL
```

in markdown, where ‘$LABEL’ is the label of an md_file or md_document, which
must be in this target’s `deps`:

``` starlark
load("@markdown//:defs.bzl", "md_document", "md_file")

md_file(
    name = "foo",
)

md_document(
    name = "bar",
    deps = [":foo"],
)
```

### Collections

``` starlark
load("@markdown//:defs.bzl", "md_collection", "md_file")

md_file(
    name = "foo",
)

md_file(
    name = "bar",
)

# md_collection generates a nicely-formatted document including all 'deps'
# under their own headings. The order of 'deps' is the order the dependencies
# appear in the document, hence 'do not sort'.
md_collection(
    name = "baz",
    author = "The Author",
    title = "The Title",
    deps = [
        # do not sort
        ":foo",
        ":bar",
    ],
)
```

## Current limitations

-   Spellchecking language is hardcoded to en_GB.
-   Lint settings can’t be customised.
-   Templates for pdf, epub etc. can’t be customised.
-   All files must be in the same workspace – no cross-workspace dependencies.
-   Catdoc must be installed as a system package, because the version in nixpkgs
    doesn’t work.
