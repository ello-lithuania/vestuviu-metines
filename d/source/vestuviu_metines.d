/**
 * Lietuviški vestuvių metinių pavadinimai – Lithuanian wedding anniversary names by year
 * (popierinės, medinės, sidabrinės, auksinės …).
 *
 * Every anniversary with gift ideas and wishes: https://vestuviumetines.lt/
 *
 * Code: MIT. Data: CC BY-SA 4.0.
 */
module vestuviu_metines;

import std.typecons : Nullable, nullable;

/// Lithuanian guide to every wedding anniversary.
enum string homepage = "https://vestuviumetines.lt/";

/// A named wedding anniversary.
struct Anniversary
{
    /// Years of marriage.
    uint years;
    /// Lithuanian name, e.g. "sidabrinės".
    string name;
    /// Other names used for the same anniversary.
    immutable(string)[] otherNames;
    /// Full name, e.g. "Sidabrinės vestuvės".
    string wedding;
    /// Page with gift ideas and wishes for this anniversary.
    string url;
}

/// All named anniversaries, ordered by years.
immutable Anniversary[] anniversaries = [
    Anniversary(1, "popierinės", [], "Popierinės vestuvės", "https://vestuviumetines.lt/1-vestuviu-metines/"),
    Anniversary(2, "medvilninės", ["kartūninės"], "Medvilninės vestuvės", "https://vestuviumetines.lt/2-vestuviu-metines/"),
    Anniversary(3, "odinės", [], "Odinės vestuvės", "https://vestuviumetines.lt/3-vestuviu-metines/"),
    Anniversary(4, "lininės", ["vaisinės"], "Lininės vestuvės", "https://vestuviumetines.lt/4-vestuviu-metines/"),
    Anniversary(5, "medinės", [], "Medinės vestuvės", "https://vestuviumetines.lt/5-vestuviu-metines/"),
    Anniversary(6, "geležinės", ["ketaus", "cukrinės"], "Geležinės vestuvės", "https://vestuviumetines.lt/6-vestuviu-metines/"),
    Anniversary(7, "varinės", ["vilnonės"], "Varinės vestuvės", "https://vestuviumetines.lt/7-vestuviu-metines/"),
    Anniversary(8, "bronzinės", ["skardinės"], "Bronzinės vestuvės", "https://vestuviumetines.lt/8-vestuviu-metines/"),
    Anniversary(9, "molinės", ["fajansinės"], "Molinės vestuvės", "https://vestuviumetines.lt/9-vestuviu-metines/"),
    Anniversary(10, "alavinės", ["rožinės"], "Alavinės vestuvės", "https://vestuviumetines.lt/10-vestuviu-metines/"),
    Anniversary(11, "plieninės", [], "Plieninės vestuvės", "https://vestuviumetines.lt/11-vestuviu-metines/"),
    Anniversary(12, "šilkinės", ["lininės"], "Šilkinės vestuvės", "https://vestuviumetines.lt/12-vestuviu-metines/"),
    Anniversary(13, "nėrinių", ["pakalnučių"], "Nėrinių vestuvės", "https://vestuviumetines.lt/13-vestuviu-metines/"),
    Anniversary(14, "dramblio kaulo", ["agato"], "Dramblio kaulo vestuvės", "https://vestuviumetines.lt/14-vestuviu-metines/"),
    Anniversary(15, "krištolinės", ["stiklinės"], "Krištolinės vestuvės", "https://vestuviumetines.lt/15-vestuviu-metines/"),
    Anniversary(18, "turkio", [], "Turkio vestuvės", "https://vestuviumetines.lt/18-vestuviu-metines/"),
    Anniversary(20, "porcelianinės", [], "Porcelianinės vestuvės", "https://vestuviumetines.lt/20-vestuviu-metines/"),
    Anniversary(25, "sidabrinės", [], "Sidabrinės vestuvės", "https://vestuviumetines.lt/25-vestuviu-metines/"),
    Anniversary(30, "perlinės", [], "Perlinės vestuvės", "https://vestuviumetines.lt/30-vestuviu-metines/"),
    Anniversary(34, "gintarinės", [], "Gintarinės vestuvės", "https://vestuviumetines.lt/34-vestuviu-metines/"),
    Anniversary(35, "koralinės", ["drobinės"], "Koralinės vestuvės", "https://vestuviumetines.lt/35-vestuviu-metines/"),
    Anniversary(40, "rubininės", [], "Rubininės vestuvės", "https://vestuviumetines.lt/40-vestuviu-metines/"),
    Anniversary(45, "safyrinės", [], "Safyrinės vestuvės", "https://vestuviumetines.lt/45-vestuviu-metines/"),
    Anniversary(46, "levandų", [], "Levandų vestuvės", "https://vestuviumetines.lt/46-vestuviu-metines/"),
    Anniversary(47, "kašmyro", [], "Kašmyro vestuvės", "https://vestuviumetines.lt/47-vestuviu-metines/"),
    Anniversary(48, "ametisto", [], "Ametisto vestuvės", "https://vestuviumetines.lt/48-vestuviu-metines/"),
    Anniversary(49, "kedro", [], "Kedro vestuvės", "https://vestuviumetines.lt/49-vestuviu-metines/"),
    Anniversary(50, "auksinės", [], "Auksinės vestuvės", "https://vestuviumetines.lt/50-vestuviu-metines/"),
    Anniversary(55, "smaragdinės", [], "Smaragdinės vestuvės", "https://vestuviumetines.lt/55-vestuviu-metines/"),
    Anniversary(60, "deimantinės", ["platininės"], "Deimantinės vestuvės", "https://vestuviumetines.lt/60-vestuviu-metines/"),
    Anniversary(65, "geležinės", [], "Geležinės vestuvės", "https://vestuviumetines.lt/65-vestuviu-metines/"),
    Anniversary(70, "palaimintosios", ["dėkingumo"], "Palaimintosios vestuvės", "https://vestuviumetines.lt/70-vestuviu-metines/"),
    Anniversary(75, "karūnuotosios", ["antrosios deimantinės"], "Karūnuotosios vestuvės", "https://vestuviumetines.lt/75-vestuviu-metines/"),
    Anniversary(80, "ąžuolinės", [], "Ąžuolinės vestuvės", "https://vestuviumetines.lt/80-vestuviu-metines/"),
];

/// The anniversary for the given number of years (null if that year has no name).
Nullable!Anniversary byYears(uint years) pure nothrow @safe
{
    foreach (a; anniversaries)
        if (a.years == years)
            return nullable(Anniversary(a.years, a.name, a.otherNames, a.wedding, a.url));
    return Nullable!Anniversary.init;
}

/// The nearest named anniversary at or after the given number of years.
Nullable!Anniversary nearestNamed(uint years) pure nothrow @safe
{
    foreach (a; anniversaries)
        if (a.years >= years)
            return nullable(Anniversary(a.years, a.name, a.otherNames, a.wedding, a.url));
    return Nullable!Anniversary.init;
}

///
unittest
{
    import std.algorithm.searching : startsWith;

    assert(byYears(25).get.name == "sidabrinės");
    assert(byYears(50).get.wedding == "Auksinės vestuvės");
    assert(byYears(16).isNull);
    assert(nearestNamed(16).get.years == 18);
    assert(nearestNamed(81).isNull);
    foreach (i, a; anniversaries)
    {
        assert(a.url.startsWith(homepage));
        if (i > 0)
            assert(anniversaries[i - 1].years < a.years);
    }
}
