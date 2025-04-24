//! logic specific to the mars weather api
use chrono::NaiveDate;

pub mod handler;
pub mod model;
pub mod router;

/// Number of Martian Sols since the Mars Curiosity Rover landing,
/// the 0-th Sol being the landing date `2012-08-06`
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct CuriositySols(pub i64);

impl CuriositySols {
    /// Curiosity Rover landing date as seen e.g. Nasa on Earth
    pub const fn landing_date() -> NaiveDate {
        NaiveDate::from_ymd_opt(2012, 8, 6).unwrap()
    }
}

impl From<NaiveDate> for CuriositySols {
    fn from(d: NaiveDate) -> Self {
        const T0: NaiveDate = CuriositySols::landing_date();
        const SCALE: f64 = 86400_f64 / 88775.245_f64;

        let days = (d - T0).num_days();
        // SCALE is < 1 so answer is bounded
        let d1 = (SCALE * (days as f64)).ceil() as i64;
        Self(d1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_sol0() {
        let d0 = NaiveDate::from_ymd_opt(2012, 8, 6).unwrap();
        let sol0 = CuriositySols::from(d0);
        assert_eq!(sol0, CuriositySols(0));
    }
}
