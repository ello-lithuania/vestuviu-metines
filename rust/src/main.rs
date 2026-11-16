//! `vestuviu-metines` – Lithuanian wedding anniversary names on the command line.

use std::env;
use std::process::ExitCode;
use vestuviu_metines::{all, by_years, nearest_named, next_anniversary, vilnius_today, Anniversary, Date};

const HELP: &str = "vestuviu-metines – lietuviški vestuvių metinių pavadinimai

Naudojimas:
  vestuviu-metines METAI            kaip vadinasi metinės, pvz. 25
  vestuviu-metines --data YYYY-MM-DD artimiausios metinės pagal vestuvių datą
  vestuviu-metines --visos          visos metinės su pavadinimais
  vestuviu-metines --version

Visos metinės su dovanų idėjomis ir sveikinimais: https://vestuviumetines.lt/
Kodas: MIT, duomenys: CC BY-SA 4.0.
";

fn describe(a: &Anniversary) -> String {
    let mut text = format!("{} m. – {} ({})", a.years, a.name, a.wedding);
    if !a.other_names.is_empty() {
        text.push_str(&format!(", dar vadinamos: {}", a.other_names.join(", ")));
    }
    text
}

fn fail(message: &str) -> ExitCode {
    eprintln!("{}", message);
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut years: Option<String> = None;
    let mut wedding: Option<String> = None;
    let mut list = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print!("{}", HELP);
                return ExitCode::SUCCESS;
            }
            "-V" | "--version" => {
                println!("vestuviu-metines {}", env!("CARGO_PKG_VERSION"));
                return ExitCode::SUCCESS;
            }
            "-a" | "--visos" => list = true,
            "-d" | "--data" => {
                i += 1;
                wedding = args.get(i).cloned();
            }
            other if other.starts_with('-') => {
                return fail(&format!("Nežinomas parametras: {}. Žr. vestuviu-metines --help", other));
            }
            other => years = Some(other.to_string()),
        }
        i += 1;
    }

    if list {
        for a in all() {
            println!("{}", describe(a));
        }
        return ExitCode::SUCCESS;
    }
    if let Some(wedding) = wedding {
        let date = match Date::parse(&wedding) {
            Some(d) => d,
            None => return fail("Data turi būti YYYY-MM-DD, pvz. 2010-06-12"),
        };
        let next = next_anniversary(date, vilnius_today());
        let when = if next.days_left == 0 { String::from("šiandien") } else { format!("po {} d.", next.days_left) };
        println!("Artimiausios metinės: {} ({}) – {} metų.", next.date, when, next.years);
        match next.anniversary {
            Some(a) => println!("{}\n{}", describe(a), a.url),
            None => {
                if let Some(n) = nearest_named(next.years) {
                    println!("Šios metinės atskiro pavadinimo neturi. Artimiausios su pavadinimu: {}", describe(n));
                }
            }
        }
        return ExitCode::SUCCESS;
    }
    if let Some(years) = years {
        let y: u32 = match years.parse() {
            Ok(y) if (1..=80).contains(&y) => y,
            _ => return fail("Metai turi būti 1–80"),
        };
        match by_years(y) {
            Some(a) => println!("{}\n{}", describe(a), a.url),
            None => {
                println!("{} metų vestuvių metinės atskiro pavadinimo neturi.", y);
                if let Some(n) = nearest_named(y) {
                    println!("Artimiausios su pavadinimu: {}\n{}", describe(n), n.url);
                }
            }
        }
        return ExitCode::SUCCESS;
    }
    print!("{}", HELP);
    ExitCode::SUCCESS
}
