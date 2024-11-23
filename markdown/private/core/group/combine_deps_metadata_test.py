import os
import os.path
from collections.abc import Mapping, Sequence
from typing import Any

from markdown.private.utils import test_utils


class TestCombineDepsMetadata(test_utils.ScriptTestCase):
    def run_script(self, metadata: Sequence[Mapping[str, str | list[str]]]) -> dict[str, Any]:  # type: ignore[override]
        metadata_args = []
        for i, d in enumerate(metadata):
            filename = os.path.join(self.tmpdir(), f"metadata_{i+1}.json")
            self.dump_json(filename, d)
            metadata_args.append(("--metadata-file", f"dep{i+1}={filename}"))

        out_file = os.path.join(self.tmpdir(), "out.json")

        super().run_script(
            args=[out_file] + [a for sublist in metadata_args for a in sublist],
        )

        return self.load_json(out_file)

    def test_empty(self) -> None:
        self.assertEqual(self.run_script([]), {})

    def test_non_empty(self) -> None:
        self.assertEqual(
            self.run_script(
                [
                    {
                        "wordcount": "10",
                        "poetry-lines": "0",
                        "lang": "en-GB",
                        "version": "foo",
                        "repo": "bar",
                        "source-hash": "quux",
                        "parsed-dates": ["2020"],
                    },
                    {
                        "wordcount": "20",
                        "poetry-lines": "10",
                        "lang": "en-US",
                        "version": "blah",
                        "repo": "yay",
                        "source-hash": "yay2",
                        "parsed-dates": ["2023/01/01"],
                    },
                ],
            ),
            {
                "dep1": {
                    "lang": "en-GB",
                    "parsed-dates": ["2020"],
                    "poetry-lines": 0,
                    "repo": "bar",
                    "source-hash": "quux",
                    "version": "foo",
                    "wordcount": 10,
                },
                "dep2": {
                    "lang": "en-US",
                    "parsed-dates": ["2023/01/01"],
                    "poetry-lines": 10,
                    "repo": "yay",
                    "source-hash": "yay2",
                    "version": "blah",
                    "wordcount": 20,
                },
            },
        )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
