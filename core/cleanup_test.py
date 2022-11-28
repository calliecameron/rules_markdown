import sys
import unittest
import utils.test_utils

PANDOC = ''
FILTER = ''

DOC = """---
title: Foo
repo: bar
---

"""


class TestCleanup(unittest.TestCase):

    def test_cleanup(self) -> None:
        j = utils.test_utils.pandoc_lua_filter(PANDOC, FILTER, DOC)
        self.assertNotIn('repo', j['meta'])


if __name__ == '__main__':
    if len(sys.argv) < 3:
        raise ValueError('Not enough args')
    PANDOC = sys.argv[1]
    del sys.argv[1]
    FILTER = sys.argv[1]
    del sys.argv[1]
    unittest.main()