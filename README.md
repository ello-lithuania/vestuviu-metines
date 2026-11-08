# Lietuviški vestuvių metinių pavadinimai (JSON / CSV)

Vestuvių metinių pavadinimai lietuviškai – nuo pirmųjų popierinių iki ąžuolinių (80 metų): 34 sukaktys su pagrindiniu ir kitais vartojamais pavadinimais. Tinka programėlėms, kalendoriams, sveikinimų generatoriams ir svetainėms.

- `vestuviu-metines.json` – `[{"metai": 25, "pavadinimas": "sidabrinės", "kiti_pavadinimai": [], "vestuves": "Sidabrinės vestuvės", "puslapis": "…"}]`;
- `vestuviu-metines.csv` – `metai,pavadinimas,kiti_pavadinimai,puslapis` (kiti pavadinimai atskirti `; `).

Pavadinimai sutikrinti su lietuviškąja Vikipedija ir santuoka.lt. Metams, kurie lietuvių tradicijoje neturi įsigalėjusio pavadinimo (pvz., 16-osios ir 17-osios), pavadinimas neišgalvotas – jų sąraše nėra.

Kiekvienų metinių reikšmė, simbolika, dovanų idėjos ir sveikinimai, metinių skaičiuoklė ir spausdinama A4 lentelė (PDF): **[vestuviumetines.lt](https://vestuviumetines.lt/)**

## Laukai

- `metai` – vestuvių metinių skaičius (1–80);
- `pavadinimas` – pagrindinis pavadinimas (pvz., „sidabrinės“);
- `kiti_pavadinimai` – kiti vartojami pavadinimai (gali būti tuščias);
- `vestuves` – pilnas pavadinimas su žodžiu „vestuvės“;
- `puslapis` – nuoroda į aprašymą, dovanų idėjas ir sveikinimus.

## Visos sukaktys

| Metai | Pavadinimas | Kiti pavadinimai |
|---|---|---|
| 1 | [popierinės](https://vestuviumetines.lt/1-vestuviu-metines/) |  |
| 2 | [medvilninės](https://vestuviumetines.lt/2-vestuviu-metines/) | kartūninės |
| 3 | [odinės](https://vestuviumetines.lt/3-vestuviu-metines/) |  |
| 4 | [lininės](https://vestuviumetines.lt/4-vestuviu-metines/) | vaisinės |
| 5 | [medinės](https://vestuviumetines.lt/5-vestuviu-metines/) |  |
| 6 | [geležinės](https://vestuviumetines.lt/6-vestuviu-metines/) | ketaus, cukrinės |
| 7 | [varinės](https://vestuviumetines.lt/7-vestuviu-metines/) | vilnonės |
| 8 | [bronzinės](https://vestuviumetines.lt/8-vestuviu-metines/) | skardinės |
| 9 | [molinės](https://vestuviumetines.lt/9-vestuviu-metines/) | fajansinės |
| 10 | [alavinės](https://vestuviumetines.lt/10-vestuviu-metines/) | rožinės |
| 11 | [plieninės](https://vestuviumetines.lt/11-vestuviu-metines/) |  |
| 12 | [šilkinės](https://vestuviumetines.lt/12-vestuviu-metines/) | lininės |
| 13 | [nėrinių](https://vestuviumetines.lt/13-vestuviu-metines/) | pakalnučių |
| 14 | [dramblio kaulo](https://vestuviumetines.lt/14-vestuviu-metines/) | agato |
| 15 | [krištolinės](https://vestuviumetines.lt/15-vestuviu-metines/) | stiklinės |
| 18 | [turkio](https://vestuviumetines.lt/18-vestuviu-metines/) |  |
| 20 | [porcelianinės](https://vestuviumetines.lt/20-vestuviu-metines/) |  |
| 25 | [sidabrinės](https://vestuviumetines.lt/25-vestuviu-metines/) |  |
| 30 | [perlinės](https://vestuviumetines.lt/30-vestuviu-metines/) |  |
| 34 | [gintarinės](https://vestuviumetines.lt/34-vestuviu-metines/) |  |
| 35 | [koralinės](https://vestuviumetines.lt/35-vestuviu-metines/) | drobinės |
| 40 | [rubininės](https://vestuviumetines.lt/40-vestuviu-metines/) |  |
| 45 | [safyrinės](https://vestuviumetines.lt/45-vestuviu-metines/) |  |
| 46 | [levandų](https://vestuviumetines.lt/46-vestuviu-metines/) |  |
| 47 | [kašmyro](https://vestuviumetines.lt/47-vestuviu-metines/) |  |
| 48 | [ametisto](https://vestuviumetines.lt/48-vestuviu-metines/) |  |
| 49 | [kedro](https://vestuviumetines.lt/49-vestuviu-metines/) |  |
| 50 | [auksinės](https://vestuviumetines.lt/50-vestuviu-metines/) |  |
| 55 | [smaragdinės](https://vestuviumetines.lt/55-vestuviu-metines/) |  |
| 60 | [deimantinės](https://vestuviumetines.lt/60-vestuviu-metines/) | platininės |
| 65 | [geležinės](https://vestuviumetines.lt/65-vestuviu-metines/) |  |
| 70 | [palaimintosios](https://vestuviumetines.lt/70-vestuviu-metines/) | dėkingumo |
| 75 | [karūnuotosios](https://vestuviumetines.lt/75-vestuviu-metines/) | antrosios deimantinės |
| 80 | [ąžuolinės](https://vestuviumetines.lt/80-vestuviu-metines/) |  |

## Pavyzdys

```js
const M = await fetch("https://raw.githubusercontent.com/ello-lithuania/vestuviu-metines/main/vestuviu-metines.json").then(r => r.json());
const metai = new Date().getFullYear() - 2001;          // vestuvių metai
const m = M.find(x => x.metai === metai);
console.log(m ? `${metai}-osios – ${m.vestuves}` : `${metai}-osios metinės`);
```

## Kaip cituoti

Žr. [`CITATION.cff`](CITATION.cff) (GitHub rodo mygtuką „Cite this repository“). Trumpai: *vestuviumetines.lt, Lietuviški vestuvių metinių pavadinimai, 2026, CC BY-SA 4.0.*

## Pataisos

Radote netikslumą ar žinote šaltinį neįtrauktam pavadinimui? Atidarykite „Issue“ nurodydami metus, pavadinimą ir šaltinį.

---

*Lithuanian wedding anniversary names (34 anniversaries, 1–80 years) as JSON/CSV. Sources: Lithuanian Wikipedia, santuoka.lt. License: CC BY-SA 4.0. Guides, gift ideas and greetings: [vestuviumetines.lt](https://vestuviumetines.lt/).*

## Licencija

Duomenys – [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Naudodami nurodykite šaltinį: [vestuviumetines.lt](https://vestuviumetines.lt/).
