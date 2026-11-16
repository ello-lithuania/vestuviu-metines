import datetime as dt
import unittest

from vestuviu_metines import core, cli


class T(unittest.TestCase):
    def test_25_sidabrines(self):
        self.assertIn("sidabr", core.by_years(25)["pavadinimas"])

    def test_50_auksines(self):
        self.assertIn("auks", core.by_years(50)["pavadinimas"])

    def test_next(self):
        d, y, rec, left = core.next_anniversary(dt.date(2000, 6, 12), today=dt.date(2026, 11, 16))
        self.assertEqual((d, y, left), (dt.date(2027, 6, 12), 27, 208))

    def test_today(self):
        d, y, rec, left = core.next_anniversary(dt.date(2001, 11, 16), today=dt.date(2026, 11, 16))
        self.assertEqual((y, left), (25, 0))

    def test_leap(self):
        d, y, rec, left = core.next_anniversary(dt.date(2008, 2, 29), today=dt.date(2026, 1, 1))
        self.assertEqual(d, dt.date(2026, 2, 28))

    def test_cli(self):
        self.assertEqual(cli.main(["25"]), 0)
        self.assertEqual(cli.main(["--visos"]), 0)
        self.assertEqual(cli.main(["--data", "2010-06-12", "--json"]), 0)
        self.assertEqual(cli.main(["99"]), 2)


if __name__ == "__main__":
    unittest.main()
