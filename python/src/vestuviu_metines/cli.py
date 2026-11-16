"""Komandinė eilutė: `vestuviu-metines`."""
from __future__ import annotations

import argparse
import datetime as dt
import json
import sys

from . import __version__
from . import core


def _name(rec):
    if not rec:
        return "pavadinimo neturi"
    extra = (" (dar vadinamos: " + ", ".join(rec["kiti_pavadinimai"]) + ")") if rec.get("kiti_pavadinimai") else ""
    return rec["vestuves"] + extra


def main(argv=None) -> int:
    p = argparse.ArgumentParser(
        prog="vestuviu-metines",
        description="Lietuviški vestuvių metinių pavadinimai. Pvz.: `vestuviu-metines 25` arba `vestuviu-metines --data 2010-06-12`.",
        epilog="Dovanos, sveikinimai ir idėjos kiekvienoms metinėms: " + core.HOMEPAGE + "  Duomenys: CC BY-SA 4.0, kodas: MIT.",
    )
    p.add_argument("metai", nargs="?", type=int, help="kelintos metinės (1–80)")
    p.add_argument("--data", "-d", metavar="YYYY-MM-DD", help="vestuvių data – parodys artimiausias metines")
    p.add_argument("--visos", "-a", action="store_true", help="visos metinės su pavadinimais")
    p.add_argument("--json", action="store_true", help="išvestis JSON formatu")
    p.add_argument("--version", action="version", version="vestuviu-metines " + __version__)
    a = p.parse_args(argv)
    if a.visos:
        rows = core.all_anniversaries()
        if a.json:
            print(json.dumps(rows, ensure_ascii=False, indent=2))
        else:
            for r in rows:
                print("%2d – %s" % (r["metai"], r["vestuves"]))
        return 0
    if a.data:
        try:
            w = dt.date.fromisoformat(a.data)
        except ValueError:
            print("Netinkama data: " + a.data + " (formatas YYYY-MM-DD)", file=sys.stderr)
            return 2
        d, years, rec, left = core.next_anniversary(w)
        if a.json:
            print(json.dumps({"data": d.isoformat(), "metai": years, "likusios_dienos": left, "irasas": rec}, ensure_ascii=False, indent=2))
        else:
            kada = "šiandien" if left == 0 else "po %d d." % left
            print("%d-osios metinės – %s (%s): %s" % (years, d.isoformat(), kada, _name(rec)))
            if not rec:
                n = core.nearest_named(years)
                if n:
                    print("Artimiausios metinės su pavadinimu: %d – %s" % (n["metai"], n["vestuves"]))
        return 0
    if a.metai is None:
        p.print_help()
        return 0
    if not 1 <= a.metai <= 80:
        print("Metai turi būti 1–80", file=sys.stderr)
        return 2
    rec = core.by_years(a.metai)
    if a.json:
        print(json.dumps({"metai": a.metai, "irasas": rec}, ensure_ascii=False, indent=2))
    else:
        print("%d metų vestuvių metinės: %s" % (a.metai, _name(rec)))
        if rec and rec.get("puslapis"):
            print(rec["puslapis"])
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
