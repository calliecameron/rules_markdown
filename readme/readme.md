---
identifier:
- scheme: DOI
  text: doi:10.234234.234/33
---

# rules_markdown

Bazel rules for markdown. Wraps pandoc and other tools for validation and conversion to other formats.

Output formats include:

* `pdf`
* `epub`, `mobi`
* `docx`, `odt`, `doc`
* `docx` in [Shunn standard manuscript format](https://www.shunn.net/format/story/)

## Prerequisites

* System packages:

  ```shell
  # Ubuntu 22.04
  sudo apt-get install g++ gcc git
  ```

* [Nix](https://nixos.org/)
* Bazel via [Bazelisk](https://github.com/bazelbuild/bazelisk)

## Setup

Create the following files in your workspace:

`.bazeliskrc`:

!include //readme:bazeliskrc

`.bazelrc`:

!include //readme:bazelrc

`WORKSPACE`: empty file

`MODULE.bazel`:

!include //readme:module_bazel

`BUILD`:

!include //readme:root_build

Initialise with:

```shell
bazel run :workspace_update
bazel run :contents_update
# Only if using md_git_repo
bazel run :git_update
```

And verify with:

```shell
bazel test :all
```

## Usage

!include //readme:basic_build

To initialise a subdirectory with a default BUILD file, run `bazel run //:new` in that directory.

Markdown files use [pandoc markdown](https://pandoc.org/MANUAL.html#pandocs-markdown), and can have the standard pandoc metadata -- title, author, date -- in a yaml front matter block.

See files under 'tests' for full examples.

### Spellchecking

!include //readme:spelling_build

A custom dictionary is a file with one word per line, containing words that should be considered correct.

### Includes

Include a file in another with:

```markdown
\!include $LABEL
```

in markdown, where '$LABEL' is the label of an md_file or md_document, which
must be in this target's `deps`:

!include //readme:include_build

### Collections

!include //readme:collection_build

## Current limitations

* On NixOS, Bazel has to be wrapped in `buildFHSEnv`, and may also need the modules `programs.nix-ld` and `services.envfs` to be enabled.
* Spellchecking language is hardcoded to en_GB.
* Lint settings can't be customised.
* Templates for pdf, epub etc. can't be customised.
* All files must be in the same workspace -- no cross-workspace dependencies.
