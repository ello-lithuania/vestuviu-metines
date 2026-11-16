//! # vestuviu-metines
//!
//! Lietuviški vestuvių metinių pavadinimai pagal metus (popierinės, medinės, sidabrinės,
//! auksinės …) ir artimiausios metinės pagal vestuvių datą. Lithuanian wedding anniversary
//! names by year and the next anniversary for a wedding date. No dependencies.
//!
//! Every anniversary with gift ideas and wishes: <https://vestuviumetines.lt/>
//!
//! ```
//! use vestuviu_metines::{by_years, next_anniversary, Date};
//!
//! assert_eq!(by_years(25).map(|a| a.name), Some("sidabrinės"));
//! let next = next_anniversary(Date::new(2010, 6, 12), Date::new(2026, 11, 16));
//! assert_eq!(next.date, Date::new(2027, 6, 12));
//! assert_eq!(next.years, 17);
//! ```
//!
//! Code: MIT. Data: CC BY-SA 4.0.
#![forbid(unsafe_code)]
#![warn(missing_docs)]

mod data;
mod date;

pub use date::{days_in_month, is_leap, vilnius_date, vilnius_today, Date};

/// Lithuanian guide to every wedding anniversary.
pub const HOMEPAGE: &str = "https://vestuviumetines.lt/";

/// A named wedding anniversary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Anniversary {
    /// Years of marriage.
    pub years: u32,
    /// Lithuanian name, e.g. `"sidabrinės"`.
    pub name: &'static str,
    /// Other names used for the same anniversary.
    pub other_names: &'static [&'static str],
    /// Full name, e.g. `"Sidabrinės vestuvės"`.
    pub wedding: &'static str,
    /// Page with gift ideas and wishes for this anniversary.
    pub url: &'static str,
}

/// The next anniversary for a wedding date (see [`next_anniversary`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NextAnniversary {
    /// Date of the anniversary (today or later; 29 February falls on 28 February in common years).
    pub date: Date,
    /// Years of marriage on that date.
    pub years: u32,
    /// The named anniversary, if that year has a name.
    pub anniversary: Option<&'static Anniversary>,
    /// Days left until the anniversary (0 = today).
    pub days_left: i64,
}

/// All named anniversaries (1–80 years), ordered by years.
pub fn all() -> &'static [Anniversary] {
    data::ANNIVERSARIES
}

/// The anniversary for the given number of years, or `None` if that year has no name.
pub fn by_years(years: u32) -> Option<&'static Anniversary> {
    data::ANNIVERSARIES.iter().find(|a| a.years == years)
}

/// The nearest named anniversary at or after the given number of years.
pub fn nearest_named(years: u32) -> Option<&'static Anniversary> {
    data::ANNIVERSARIES.iter().find(|a| a.years >= years)
}

fn same_day(year: i64, wedding: Date) -> Date {
    if wedding.month == 2 && wedding.day == 29 && !is_leap(year) {
        Date::new(year, 2, 28)
    } else {
        Date::new(year, wedding.month, wedding.day)
    }
}

/// The next anniversary (today or later) for a wedding date.
pub fn next_anniversary(wedding: Date, today: Date) -> NextAnniversary {
    let mut date = same_day(today.year, wedding);
    if date < today {
        date = same_day(today.year + 1, wedding);
    }
    let years = (date.year - wedding.year).max(0) as u32;
    NextAnniversary {
        date,
        years,
        anniversary: by_years(years),
        days_left: date.to_days() - today.to_days(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names() {
        assert_eq!(by_years(1).map(|a| a.name), Some("popierinės"));
        assert_eq!(by_years(50).map(|a| a.wedding), Some("Auksinės vestuvės"));
        assert!(by_years(16).is_none());
        assert_eq!(nearest_named(16).map(|a| a.years), Some(18));
        assert!(nearest_named(81).is_none());
        assert!(all().windows(2).all(|w| w[0].years < w[1].years));
        assert!(all().iter().all(|a| a.url.starts_with(HOMEPAGE)));
    }

    #[test]
    fn next() {
        let n = next_anniversary(Date::new(2001, 11, 16), Date::new(2026, 11, 16));
        assert_eq!((n.years, n.days_left), (25, 0));
        assert_eq!(n.anniversary.map(|a| a.name), Some("sidabrinės"));
        let leap = next_anniversary(Date::new(2024, 2, 29), Date::new(2026, 11, 16));
        assert_eq!(leap.date, Date::new(2027, 2, 28));
        assert_eq!(leap.years, 3);
    }
}
