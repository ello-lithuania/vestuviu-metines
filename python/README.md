# vestuviu-metines

Lietuviški vestuvių metinių pavadinimai (popierinės, medinės, sidabrinės, auksinės…) – Python biblioteka ir komandinė eilutė.

**Dovanų idėjos, sveikinimai ir tradicijos kiekvienoms metinėms – [vestuviumetines.lt](https://vestuviumetines.lt/).**

```
$ vestuviu-metines 25
25 metų vestuvių metinės: Sidabrinės vestuvės
https://vestuviumetines.lt/25-vestuviu-metines/

$ vestuviu-metines --data 2001-11-16
25-osios metinės – 2026-11-16 (šiandien): Sidabrinės vestuvės

$ vestuviu-metines --visos
```

```python
from vestuviu_metines import by_years, next_anniversary
by_years(40)["vestuves"]   # 'Rubininės vestuvės'
```

Licencija: kodas – MIT, duomenys – CC BY-SA 4.0 (šaltiniai: lietuviškoji Vikipedija, santuoka.lt; sudarė [vestuviumetines.lt](https://vestuviumetines.lt/)).

*Lithuanian wedding anniversary names (1–80 years) as a small Python library and CLI. More: [vestuviumetines.lt](https://vestuviumetines.lt/).*
