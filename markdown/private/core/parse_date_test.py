import os
import os.path
from typing import Any

from markdown.private.utils import test_utils


class TestParseDate(test_utils.ScriptTestCase):
    def run_script(self, date: str) -> dict[str, Any]:  # type: ignore[override]
        in_file = os.path.join(self.tmpdir(), "in.json")
        self.dump_json(in_file, {"date": date})

        out_file = os.path.join(self.tmpdir(), "out.json")

        super().run_script(
            args=[
                in_file,
                out_file,
            ],
        )

        return self.load_json(out_file)

    def test_parse_date(self) -> None:
        self.assertEqual(
            self.run_script(""),
            {"parsed-dates": []},
        )

        self.assertEqual(
            self.run_script("no dates"),
            {"parsed-dates": []},
        )

        self.assertEqual(
            self.run_script("2022"),
            {"parsed-dates": ["2022"]},
        )

        self.assertEqual(
            self.run_script("from August 2020 to 1 March 2023"),
            {"parsed-dates": ["2020/08", "2023/03/01"]},
        )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
