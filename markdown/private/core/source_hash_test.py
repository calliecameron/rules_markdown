import os
import os.path
from collections.abc import Mapping
from typing import Any

from markdown.private.utils import test_utils


class TestSourceHash(test_utils.ScriptTestCase):
    def run_script(  # type: ignore[override]
        self,
        src: str,
        deps_metadata: Mapping[str, Mapping[str, str | list[str]]],
    ) -> dict[str, Any]:
        src_file = os.path.join(self.tmpdir(), "src.md")
        self.dump_file(src_file, src)

        deps_metadata_file = os.path.join(self.tmpdir(), "deps_metadata.json")
        self.dump_json(deps_metadata_file, deps_metadata)

        metadata_out_file = os.path.join(self.tmpdir(), "metadata_out.json")

        super().run_script(
            args=[
                src_file,
                deps_metadata_file,
                metadata_out_file,
            ],
        )

        return self.load_json(metadata_out_file)

    def test_script(self) -> None:
        metadata_out = self.run_script(
            "foo bar\n",
            {
                "dep1": {
                    "wordcount": "10",
                    "poetry-lines": "0",
                    "lang": "en-GB",
                    "version": "foo",
                    "repo": "bar",
                    "source-hash": "1",
                    "parsed-dates": [],
                },
                "dep2": {
                    "wordcount": "20",
                    "poetry-lines": "10",
                    "lang": "en-US",
                    "version": "blah",
                    "repo": "yay",
                    "source-hash": "3",
                    "parsed-dates": [],
                },
            },
        )

        self.assertEqual(
            metadata_out,
            {
                "source-hash": "0ce6eeb51f34f9ba235c549dc38f794d",
            },
        )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()