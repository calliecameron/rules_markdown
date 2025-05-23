import os
import os.path

from markdown.private.utils import test_utils


class TestWriteGroupPublicationsScript(test_utils.ScriptTestCase):
    def test_write_group_publications_script(self) -> None:
        out_file = os.path.join(self.tmpdir(), "out.sh")

        self.run_script(
            args=[
                "foo",
                "bar",
                out_file,
            ],
        )

        self.assertEqual(
            self.load_file(out_file),
            """#!/bin/bash

set -eu

FILE_TO_OPEN="${0}.runfiles/foo/bar"

xdg-open "${FILE_TO_OPEN}"
""",
        )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
