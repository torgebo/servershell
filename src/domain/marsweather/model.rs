use crate::model::APIGResp;
use chrono::NaiveDate;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Deserialize, Serialize)]
pub struct DateRequest {
    pub date: NaiveDate,
}

/// Outbound response from our server
pub type OutboundResp = APIGResp<DateRequest, Option<Observation>>;

impl OutboundResp {
    pub fn new(date: NaiveDate, response: Option<Observation>) -> Self {
        Self {
            request: DateRequest { date },
            data: response,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct NasasResponse {
    pub soles: Vec<Observation>,
}

/// Observations from a single Sol
///
/// TODO:
/// parse all fields? some seem a little funky.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Observation {
    pub id: StringI64,
    pub terrestrial_date: NaiveDate,
    pub sol: StringI64,
    pub ls: StringI64,
    pub season: String,
    pub min_temp: StringI64,
    pub max_temp: StringI64,
    pub pressure: StringI64,
    pub pressure_string: String,
    pub abs_humidity: String,
    pub wind_speed: String,
    pub wind_direction: String,
    pub atmo_opacity: String,
    pub sunrise: String,
    pub sunset: String,
    pub local_uv_irradiance_index: String,
    pub min_gts_temp: StringI64,
    pub max_gts_temp: StringI64,
}

/// custom type for decoding integers stored as strings
/// as well as handling e.g. use of value "--" to signal absence of data
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct StringI64(pub Option<i64>);

impl<'de> Deserialize<'de> for StringI64 {
    fn deserialize<D>(deserializer: D) -> Result<StringI64, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(StringI64Visitor)
    }
}

impl Serialize for StringI64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            None => serializer.serialize_none(),
            Some(i) => serializer.serialize_some::<i64>(&i),
        }
    }
}

struct StringI64Visitor;

impl Visitor<'_> for StringI64Visitor {
    type Value = StringI64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an i64 wrapped in String quotes")
    }

    /// we simply attempt string conversion, and fall back on None when it fails
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value.parse::<i64>() {
            Ok(i) => Ok(StringI64(Some(i))),
            Err(_e) => Ok(StringI64(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_observation() {
        let obs_contents = r#"{
    "id": "230",
    "terrestrial_date": "2012-11-04",
    "sol": "88",
    "ls": "200",
    "season": "Month 7",
    "min_temp": "-70",
    "max_temp": "-2",
    "pressure": "811",
    "pressure_string": "Higher",
    "abs_humidity": "--",
    "wind_speed": "--",
    "wind_direction": "--",
    "atmo_opacity": "Sunny",
    "sunrise": "05:18",
    "sunset": "17:24",
    "local_uv_irradiance_index": "Very_High",
    "min_gts_temp": "-82",
    "max_gts_temp": "15"
}"#;
        let _obs: Observation = serde_json::from_str(&obs_contents).expect("deserializes");
    }

    #[test]
    fn json_decode_stringi64() {
        #[derive(Deserialize, Serialize)]
        struct PlaceHolder {
            pub i: StringI64,
        }

        let jsonstrs = vec![r#"{"i": "3"}"#];

        for (i, jsons) in jsonstrs.iter().enumerate() {
            if let Err(e) = serde_json::from_str::<PlaceHolder>(&jsons) {
                assert!(
                    false,
                    "json string index #{}: unable to deserialize: error {}",
                    i, e
                );
            }
        }
    }
}
