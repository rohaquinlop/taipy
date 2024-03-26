from typycheck import (
    rust,
)
import unittest


class TestRust(unittest.TestCase):
    def test_sum_as_string(self):
        self.assertEqual("3", "3")
