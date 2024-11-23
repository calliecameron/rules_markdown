import os
import os.path
import subprocess
from collections.abc import Mapping
from typing import Any

from markdown.private.utils import test_utils


class TestValidateOutputMetadata(test_utils.ScriptTestCase):
    def run_script(  # type: ignore[override]
        self,
        content: Mapping[str, Any],
    ) -> dict[str, Any]:
        in_file = os.path.join(self.tmpdir(), "in.json")
        self.dump_json(in_file, content)

        out_file = os.path.join(self.tmpdir(), "out.json")

        super().run_script(
            args=[
                in_file,
                out_file,
            ],
        )

        return self.load_json(out_file)

    def test_good(self) -> None:
        self.assertEqual(
            self.run_script(
                {
                    "wordcount": 10,
                    "poetry-lines": "10",
                    "lang": "en-GB",
                    "version": "foo",
                    "repo": "bar",
                    "source-hash": "quux",
                    "parsed-dates": [],
                },
            ),
            {
                "lang": "en-GB",
                "parsed-dates": [],
                "poetry-lines": 10,
                "repo": "bar",
                "source-hash": "quux",
                "version": "foo",
                "wordcount": 10,
            },
        )

    def test_fails(self) -> None:
        with self.assertRaises(subprocess.CalledProcessError):
            self.run_script({})


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
