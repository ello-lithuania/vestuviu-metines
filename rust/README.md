# vestuviu-metines (Rust)

Lietuviški vestuvių metinių pavadinimai pagal metus ir artimiausios metinės pagal vestuvių datą – Rust biblioteka ir komandinės eilutės įrankis, be priklausomybių.
Lithuanian wedding anniversary names by year, and the next anniversary for a wedding date.

Kiekvienos metinės su dovanų idėjomis ir sveikinimais: **[vestuviumetines.lt](https://vestuviumetines.lt/)**

```rust
use vestuviu_metines::{by_years, next_anniversary, vilnius_today, Date};

let silver = by_years(25).unwrap();          // "sidabrinės", "Sidabrinės vestuvės"
let next = next_anniversary(Date::new(2010, 6, 12), vilnius_today());
println!("{} – {} m.", next.date, next.years);
```

```sh
cargo install vestuviu-metines
vestuviu-metines 25                  # sidabrinės
vestuviu-metines --data 2010-06-12   # artimiausios metinės
vestuviu-metines --visos             # visas sąrašas
```

The same data as JSON / CSV and as a Python package: [github.com/ello-lithuania/vestuviu-metines](https://github.com/ello-lithuania/vestuviu-metines).

Code: MIT. Data: CC BY-SA 4.0.
