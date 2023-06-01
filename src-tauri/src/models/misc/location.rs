use crate::{prelude::*, utils::HasId};
use chrono::{DateTime, Utc};
use uuid::Uuid;
#[derive(Clone, Default, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct Location {
    #[serde(skip)]
    id: Uuid,
    country: String,
    region: String,
    city: String,
    #[serde(with = "ts_seconds")]
    timezone: DateTime<Utc>,
}

impl Location {
    pub async fn new(country: &str, region: &str, city: &str, timezone: DateTime<Utc>) -> Self {
        Self::load(Uuid::new_v4(), country, region, city, timezone).await
    }

    pub async fn load(
        id: Uuid,
        country: &str,
        region: &str,
        city: &str,
        timezone: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            country: country.to_string(),
            region: region.to_string(),
            city: city.to_string(),
            timezone,
        }
    }
}

///
/// We Hide Business Logic behind separate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs (or just strip it out of said logic).
///
/// Import Logic: ``use <path>::<struct>::logic::*;``
///
mod logic {
    impl super::Location {}
}

/// Boilerplate
impl Location {
    getter_ref!( { async } id: &Uuid, { async } country: &str, { async } region: &str, { async } city: &str, { async } timezone: &DateTime<Utc>);
    getter_mut!( { async } id: &mut Uuid, { async } country: &mut String, { async } region: &mut String, { async } city: &mut String);
    setter!( { async } id: Uuid, { async } country: &str, { async } region: &str, { async } city: &str );
    getter!( { async } id: Uuid);
}

impl HasId for Location {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
