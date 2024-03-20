from typycheck import (
    rust,
)
import unittest


class TestRust(unittest.TestCase):
    def test_sum_as_string(self):
        self.assertEqual(rust.sum_as_string(1, 2), "3")
