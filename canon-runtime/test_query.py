#!/usr/bin/env python3
"""Unit tests for the read-only canon query module (#896)."""

from __future__ import annotations

import unittest

from query import CanonQuery


class CanonQueryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.q = CanonQuery()

    def test_emerald_hex(self) -> None:
        row = self.q.get_tablet_by_color("#50C878")
        self.assertIsNotNone(row)
        assert row is not None
        self.assertEqual(row["id"], "emerald")
        self.assertEqual(row["color"]["hex"].upper(), "#50C878")

    def test_terra_bistre(self) -> None:
        row = self.q.get_tablet_by_color("#3D2B1F")
        self.assertIsNotNone(row)
        assert row is not None
        self.assertEqual(row["id"], "terra")
        self.assertEqual(row["color"]["hex"].upper(), "#3D2B1F")

    def test_unknown_id_is_clean_miss(self) -> None:
        self.assertIsNone(self.q.get_tablet("not-a-tablet"))
        self.assertIsNone(self.q.get_constraints("not-a-tablet"))
        self.assertIsNone(self.q.get_tablet_by_color("#000001"))

    def test_citrine_cap_nonempty(self) -> None:
        self.assertTrue(self.q.get_constraints("citrine"))
        self.assertTrue(self.q.get_prohibitions("citrine"))

    def test_amber_cap_explicitly_empty(self) -> None:
        self.assertEqual(self.q.get_constraints("amber"), [])
        self.assertEqual(self.q.get_affordances("amber"), [])
        self.assertEqual(self.q.get_prohibitions("amber"), [])

    def test_element_filter(self) -> None:
        earth = self.q.get_tablets_by_element("Earth")
        ids = {row["id"] for row in earth}
        self.assertIn("emerald", ids)
        self.assertIn("terra", ids)


if __name__ == "__main__":
    unittest.main()
