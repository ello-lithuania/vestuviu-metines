"""Vestuvių metinių pavadinimai: pagal metus ir pagal vestuvių datą."""
from __future__ import annotations

import datetime as _dt
import json
from importlib import resources

HOMEPAGE = "https://vestuviumetines.lt/"


def _load():
    with resources.files(__package__).joinpath("data/metines.json").open(encoding="utf-8") as f:
        return json.load(f)


_DATA = _load()
_BY = {d["metai"]: d for d in _DATA}


def all_anniversaries():
    """Visos metinės su pavadinimais (1–80 m.)."""
    return list(_DATA)


def by_years(years: int):
    """Metinių įrašas pagal metus arba None, jei tie metai pavadinimo neturi."""
    return _BY.get(int(years))


def nearest_named(years: int):
    """Artimiausios vėlesnės metinės, turinčios pavadinimą."""
    for y in range(int(years), 81):
        if y in _BY:
            return _BY[y]
    return None


def _same_day(year: int, wedding: _dt.date) -> _dt.date:
    try:
        return wedding.replace(year=year)
    except ValueError:  # vasario 29 d.
        return _dt.date(year, 2, 28)


def next_anniversary(wedding: _dt.date, today: _dt.date | None = None):
    """Artimiausios (šiandien arba vėliau) metinės: (data, metai, įrašas arba None, likusios dienos)."""
    today = today or _dt.date.today()
    d = _same_day(today.year, wedding)
    if d < today:
        d = _same_day(today.year + 1, wedding)
    years = d.year - wedding.year
    return d, years, by_years(years), (d - today).days
