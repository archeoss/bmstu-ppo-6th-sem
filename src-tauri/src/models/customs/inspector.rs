use uuid::Uuid;

use crate::prelude::*;

#[derive(Clone, Default, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct Inspector {
    name: String,
    rank: String,
    post: String,
}

impl Inspector {
    pub async fn new(name: &str, post: &str, rank: &str) -> Self {
        Self {
            name: name.to_string(),
            post: post.to_string(),
            rank: rank.to_string(),
        }
    }
}
///
/// We Hide Business Logic behind separate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs (or just strip it out of said logic).
///
/// Import Logic: use <path>::<struct>::logic::*;
///
mod logic {
    impl super::Inspector {}
}

/// Boilerplate
impl Inspector {
    getter_ref!( { async } name: &str, { async } rank: &str, { async } post: &str);
    getter_mut!( { async } name: &mut String, { async } rank: &mut String, { async } post: &mut String);
    setter!( { async } name: &str, { async } rank: &str, { async } post: &str);
}
