use uuid::Uuid;

use crate::prelude::*;

#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct Operator {
    id: Uuid,
    name: String,
    post: String,
}

impl Operator {
    pub async fn new(name: &str, post: &str) -> Self {
        Self::load(Uuid::new_v4(), name, post).await
    }

    pub async fn load(id: Uuid, name: &str, post: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            post: post.to_string(),
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
    trait Logic {
        // fn async change_params(&self,) {
        // }
    }

    impl Logic for super::Operator {}
}

/// Boilerplate
impl Operator {
    getter_ref!( { async } id: &Uuid, { async } name: &str, { async } post: &str);
    getter_mut!( { async } id: &mut Uuid, { async } name: &mut String, { async } post: &mut String);
    setter!( { async } id: Uuid, { async } name: &str, { async } post: &str);
    getter!( { async } id: Uuid);
}
