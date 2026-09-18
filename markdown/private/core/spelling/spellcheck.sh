#!/bin/bash

set -euo pipefail

function usage() {
    echo "Usage: $(basename "${0}") perl hunspell dict_dir locale_archive custom_dict_file in_file out_file language"
    exit 1
}

test -z "${1:-}" && usage
PERL="${1}"
test -z "${2:-}" && usage
HUNSPELL="${2}"
test -z "${3:-}" && usage
DICT_DIR="${3}"
test -z "${4:-}" && usage
LOCALE_ARCHIVE="${4}"
test -z "${5:-}" && usage
CUSTOM_DICT_FILE="${5}"
test -z "${6:-}" && usage
IN_FILE="${6}"
test -z "${7:-}" && usage
OUT_FILE="${7}"
test -z "${8:-}" && usage
LANGUAGE="${8}"

# Hunspell doesn't like single curly quotes
# shellcheck disable=SC1112,SC2016
OUTPUT="$(
    "${PERL}" -pe 's/(\W)‘/$1/g;s/’(\W)/$1/g;s/^‘//;s/’$//;' <"${IN_FILE}" |
        HOME="${PWD}" LC_ALL="${LANGUAGE}.UTF-8" LOCALE_ARCHIVE="${LOCALE_ARCHIVE}" DICPATH="${DICT_DIR}" "${HUNSPELL}" -d "${LANGUAGE}" -p "${CUSTOM_DICT_FILE}" -l |
        LC_ALL=C sort --ignore-case |
        uniq
)"

if [ -n "${OUTPUT}" ]; then
    echo "ERROR: found misspelled words; correct them or add them to the dictionary:" >&2
    echo >&2
    echo "${OUTPUT}" >&2
    echo >&2
    exit 1
fi

echo 'OK' >"${OUT_FILE}"
