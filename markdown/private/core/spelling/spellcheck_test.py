import os
import os.path
import subprocess
from collections.abc import Sequence

from markdown.private.utils import test_utils

DOC = """The Title

Test text baz Quux shouldn’t fail
"""  # noqa: RUF001


class TestWriteDictionary(test_utils.ScriptTestCase):
    def run_spellcheck(self, doc: str, dictionary: Sequence[str]) -> str:
        perl = self.test_args()[0]
        hunspell = self.test_args()[1]
        locale_archive = self.test_args()[2]
        dict_dir = os.path.dirname(self.test_args()[3])

        in_file = os.path.join(self.tmpdir(), "in.md")
        self.dump_file(in_file, doc)

        dict_file = os.path.join(self.tmpdir(), "in.dic")
        self.dump_file(dict_file, "\n".join(dictionary) + "\n")

        out_file = os.path.join(self.tmpdir(), "out.txt")

        self.run_script(
            args=[
                perl,
                hunspell,
                dict_dir,
                locale_archive,
                dict_file,
                in_file,
                out_file,
                "en_GB",
            ],
        )

        return self.load_file(out_file)

    def test_spellcheck(self) -> None:
        self.assertEqual(self.run_spellcheck(DOC, ["baz", "Quux"]), "OK\n")

    def test_spellcheck_fails(self) -> None:
        try:
            self.run_spellcheck(DOC, [])
            self.fail()
        except subprocess.CalledProcessError as e:
            self.assertEqual(
                e.stderr,
                """ERROR: found misspelled words; correct them or add them to the dictionary:

baz
Quux

""",
            )


if __name__ == "__main__":
    test_utils.ScriptTestCase.main()
