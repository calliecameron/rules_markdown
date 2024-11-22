import datetime
from collections.abc import Iterator, Mapping, Sequence
from typing import Annotated, Any

from pydantic import BaseModel, ConfigDict, Field, RootModel, field_validator
from pydantic.functional_validators import BeforeValidator

from markdown.private.utils.publications import Publications


def _validate_parsed_date(d: str) -> None:
    patterns = ("%Y", "%Y/%m", "%Y/%m/%d")
    for pattern in patterns:
        try:
            datetime.datetime.strptime(d, pattern)  # noqa: DTZ007
        except ValueError:  # noqa: PERF203
            pass
        else:
            return
    raise ValueError("Must match YYYY, YYYY/MM or YYYY/MM/DD format")


def _validate_sorted_date_set_field(v: Any) -> Any:  # noqa: ANN401
    if not isinstance(v, Sequence):
        raise ValueError("Must be a sequence of strings in YYYY, YYYY/MM or YYYY/MM/DD format")
    out = []
    for i in v:
        if not isinstance(i, str):
            raise ValueError("Must be a string in YYYY, YYYY/MM or YYYY/MM/DD format")
        _validate_parsed_date(i)
        out.append(i)
    if out != sorted(frozenset(out)):
        raise ValueError("Elements must be unique and sorted")
    return out


ParsedDateSet = Annotated[
    Sequence[str],
    BeforeValidator(_validate_sorted_date_set_field),
]


class _BaseModel(BaseModel):
    model_config = ConfigDict(
        frozen=True,
        strict=True,
        extra="forbid",
        alias_generator=lambda s: s.replace("_", "-"),
    )


class SourceHash(_BaseModel):
    source_hash: str


class ParsedDates(_BaseModel):
    parsed_dates: ParsedDateSet


class Identifier(_BaseModel):
    scheme: str
    text: str


class InputMetadata(_BaseModel):
    title: str = ""
    author: Sequence[str] = []
    date: str = ""
    notes: str = ""
    finished: bool = False
    publications: Publications = Publications.model_construct([])
    identifier: Sequence[Identifier] = []

    @field_validator("author", mode="before")
    @classmethod
    def _convert_author(cls, v: Any) -> Any:  # noqa: ANN401
        if isinstance(v, str):
            return [v]
        if isinstance(v, list) and not v:
            raise ValueError(
                f"metadata item 'author' must be a non-empty list of string or a string; got "
                f"{v}",
            )
        return v


class OutputMetadata(InputMetadata):
    wordcount: int = Field(strict=False, ge=0)
    poetry_lines: int = Field(strict=False, ge=0)
    lang: str
    version: str
    repo: str
    source_hash: str
    parsed_dates: ParsedDateSet


class MetadataMap(RootModel[Mapping[str, OutputMetadata]], Mapping[str, OutputMetadata]):
    model_config = ConfigDict(
        frozen=True,
        strict=True,
    )

    def __getitem__(self, k: str) -> OutputMetadata:
        return self.root[k]

    def __iter__(self) -> Iterator[str]:  # type: ignore[override]
        return iter(self.root)

    def __len__(self) -> int:
        return len(self.root)
