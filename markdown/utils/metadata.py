from collections.abc import Mapping, Sequence
from typing import Any

from pydantic import BaseModel, ConfigDict, Field, RootModel, field_validator

from markdown.utils.publications import Publications


class _BaseModel(BaseModel):
    model_config = ConfigDict(
        frozen=True,
        strict=True,
        extra="forbid",
        alias_generator=lambda s: s.replace("_", "-"),
    )


class Version(_BaseModel):
    docversion: str
    repo: str


class VersionMetadata(_BaseModel):
    docversion: str
    repo: str
    subject: str


class SourceHash(_BaseModel):
    source_hash: str


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
    docversion: str
    repo: str
    subject: str
    source_hash: str


class MetadataMap(RootModel[Mapping[str, OutputMetadata]]):
    model_config = ConfigDict(
        frozen=True,
        strict=True,
    )

    @property
    def metadata(self) -> Mapping[str, OutputMetadata]:
        return self.root
