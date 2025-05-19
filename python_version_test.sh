#!/bin/bash

set -eu

function usage() {
    echo "Usage: $(basename "${0}") module_bazel python_version_file pyproject_toml python_bzl"
    exit 1
}

test -z "${1:-}" && usage
MODULE_BAZEL="${1}"
test -z "${2:-}" && usage
PYTHON_VERSION_FILE="${2}"
test -z "${3:-}" && usage
PYPROJECT_TOML="${3}"
test -z "${4:-}" && usage
PYTHON_BZL="${4}"

PYTHON_VERSION="$(grep '^PYTHON_VERSION = ' "${MODULE_BAZEL}" | grep -E -o '[0-9]+\.[0-9]+\.[0-9]+')"

FAIL=''

EXPECTED_VERSION_FILE_VERSION="${PYTHON_VERSION}"
ACTUAL_VERSION_FILE_VERSION="$(cat "${PYTHON_VERSION_FILE}")"

if [ "${ACTUAL_VERSION_FILE_VERSION}" != "${EXPECTED_VERSION_FILE_VERSION}" ]; then
    echo "contents of .python-version must be ${EXPECTED_VERSION_FILE_VERSION}; got ${ACTUAL_VERSION_FILE_VERSION}"
    FAIL='t'
fi

EXPECTED_MYPY_VERSION="$(echo "${PYTHON_VERSION}" | grep -E -o '^[0-9]+\.[0-9]+')"
ACTUAL_MYPY_VERSION="$(grep '^python_version = ' "${PYPROJECT_TOML}" | grep -E -o '[0-9]+\.[0-9]+')"

if [ "${ACTUAL_MYPY_VERSION}" != "${EXPECTED_MYPY_VERSION}" ]; then
    echo "tools.mypy python_version in pyproject.toml must be ${EXPECTED_MYPY_VERSION}; got ${ACTUAL_MYPY_VERSION}"
    FAIL='t'
fi

EXPECTED_RUFF_VERSION="py$(echo "${PYTHON_VERSION}" | grep -E -o '^[0-9]+\.[0-9]+' | sed 's|\.||g')"
ACTUAL_RUFF_VERSION="$(grep '^target-version = ' "${PYPROJECT_TOML}" | grep -E -o 'py[0-9]+')"

if [ "${ACTUAL_RUFF_VERSION}" != "${EXPECTED_RUFF_VERSION}" ]; then
    echo "tools.ruff target-version in pyproject.toml must be ${EXPECTED_RUFF_VERSION}; got ${ACTUAL_RUFF_VERSION}"
    FAIL='t'
fi

EXPECTED_PYTHON_BZL_VERSION="${PYTHON_VERSION}"
ACTUAL_PYTHON_BZL_VERSION="$(grep '^PYTHON_VERSION = ' "${PYTHON_BZL}" | grep -E -o '[0-9]+\.[0-9]+\.[0-9]+')"

if [ "${ACTUAL_PYTHON_BZL_VERSION}" != "${EXPECTED_PYTHON_BZL_VERSION}" ]; then
    echo "PYTHON_VERSION in markdown/private/support/python/defs.bzl must be ${EXPECTED_PYTHON_BZL_VERSION}; got ${ACTUAL_PYTHON_BZL_VERSION}"
    FAIL='t'
fi

if [ -n "${FAIL}" ]; then
    exit 1
fi
exit 0
