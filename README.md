# Lietuviški vestuvių metinių pavadinimai (JSON / CSV)

Vestuvių metinių pavadinimai lietuviškai – nuo pirmųjų popierinių iki ąžuolinių (80 metų): 34 sukaktys su pagrindiniu ir kitais vartojamais pavadinimais. Tinka programėlėms, kalendoriams, sveikinimų generatoriams ir svetainėms.

- `vestuviu-metines.json` – `[{"metai": 25, "pavadinimas": "sidabrinės", "kiti_pavadinimai": [], "vestuves": "Sidabrinės vestuvės", "puslapis": "…"}]`;
- `vestuviu-metines.csv` – `metai,pavadinimas,kiti_pavadinimai,puslapis` (kiti pavadinimai atskirti `; `).

Pavadinimai sutikrinti su lietuviškąja Vikipedija ir santuoka.lt. Metams, kurie lietuvių tradicijoje neturi įsigalėjusio pavadinimo (pvz., 16-osios ir 17-osios), pavadinimas neišgalvotas – jų sąraše nėra.

Kiekvienų metinių reikšmė, simbolika, dovanų idėjos ir sveikinimai, metinių skaičiuoklė ir spausdinama A4 lentelė (PDF): **[vestuviumetines.lt](https://vestuviumetines.lt/)**

## Pavyzdys

```js
const M = await fetch("https://raw.githubusercontent.com/ello-lithuania/vestuviu-metines/main/vestuviu-metines.json").then(r => r.json());
const metai = new Date().getFullYear() - 2001;          // vestuvių metai
const m = M.find(x => x.metai === metai);
console.log(m ? `${metai}-osios – ${m.vestuves}` : `${metai}-osios metinės`);
```

## Licencija

Duomenys – [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/). Naudodami nurodykite šaltinį: [vestuviumetines.lt](https://vestuviumetines.lt/).
