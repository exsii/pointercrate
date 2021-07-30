use crate::cistring::CiString;
use derive_more::Constructor;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::etag::Taggable;
pub use paginate::{NationalityRankingPagination, RankedNation};

mod get;
mod paginate;

#[derive(Debug, PartialEq, Eq, Serialize, Hash, Constructor)]
pub struct Nationality {
    #[serde(rename = "country_code")]
    pub iso_country_code: String,
    pub nation: CiString,
    pub subdivision: Option<Subdivision>,
}

#[derive(Debug, Serialize, Hash)]
pub struct BestRecord {
    id: i32,
    demon: String,
    position: i16,
    progress: i16,
    players: Vec<String>,
}

#[derive(Debug, Serialize, Hash)]
pub struct MiniDemon {
    id: i32,
    demon: String,
    position: i16,
    player: String,
}

#[derive(Debug, Serialize, Hash)]
pub struct MiniDemonWithPlayers {
    id: i32,
    demon: String,
    position: i16,
    players: Vec<String>,
}

#[derive(Debug, Hash, Serialize)]
pub struct NationalityRecord {
    pub nation: Nationality,

    #[serde(rename = "records")]
    pub best_records: Vec<BestRecord>,
    pub created: Vec<MiniDemonWithPlayers>,
    pub verified: Vec<MiniDemon>,
    pub published: Vec<MiniDemon>,
}

impl Taggable for NationalityRecord {}

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Hash, Constructor)]
pub struct Subdivision {
    pub iso_code: String,
    pub name: CiString,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum Continent {
    Asia,
    Europe,
    AustraliaAndOceania,
    Africa,
    NorthAmerica,
    SouthAmerica,
    MiddleAmerica,
}

impl Continent {
    pub fn to_sql(&self) -> String {
        match self {
            Continent::Asia => "Asia",
            Continent::Europe => "Europe",
            Continent::AustraliaAndOceania => "Australia and Oceania",
            Continent::Africa => "Africa",
            Continent::NorthAmerica => "North America",
            Continent::SouthAmerica => "South America",
            Continent::MiddleAmerica => "Central America",
        }
        .to_owned()
    }
}

impl<'de> Deserialize<'de> for Continent {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?.to_lowercase();

        match &string[..] {
            "asia" => Ok(Continent::Asia),
            "europe" => Ok(Continent::Europe),
            "australia" => Ok(Continent::AustraliaAndOceania),
            "africa" => Ok(Continent::Africa),
            "north america" => Ok(Continent::NorthAmerica),
            "south america" => Ok(Continent::SouthAmerica),
            "central america" => Ok(Continent::MiddleAmerica),
            _ =>
                Err(serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&string),
                    &"'Asia', 'Europe', 'Australia', 'Africa', 'North America', 'South America' or 'Central America'",
                )),
        }
    }
}

impl Serialize for Continent {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match self {
            Continent::Asia => "asia",
            Continent::Europe => "europe",
            Continent::AustraliaAndOceania => "australia",
            Continent::Africa => "africa",
            Continent::NorthAmerica => "north america",
            Continent::SouthAmerica => "south america",
            Continent::MiddleAmerica => "central america",
        })
    }
}
