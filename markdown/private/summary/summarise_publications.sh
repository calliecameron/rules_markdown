#!/bin/bash
# Auto-generated

set -eu

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "${THIS_DIR}"

"${THIS_DIR}/.markdown_summary/refresh"

bazel run .markdown_summary:contents_publications
