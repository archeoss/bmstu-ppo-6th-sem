use uuid::Uuid;

use crate::{models::misc::location::Location, prelude::*};
#[derive(Debug)]
struct Client<'a> {
    id: Uuid,
    name: String,
    location: Option<&'a Location>,
}
///
/// We Hide Business Logic behind separate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs (or just strip it out of said logic).
///
/// Import Logic: use <path>::<struct>::logic::*;
///
mod logic {}

/// Boilerplate
impl<'a> Client<'a> {
    getter_ref!( { async } id: &i64, { async } name: &str);
    getter_mut!( { async } id: &mut i64, { async } name: &mut String);
    setter!( { async } id: i64, { async } name: &str);
    getter!( { async } id: i64);
}
