use uuid::Uuid;

use crate::{
    models::{declaration::Declaration, misc::location::Location},
    prelude::*,
};

use futures::stream;
use futures::StreamExt;

use crate::errors::declaration::Err as DErr;
use crate::models::participants::Participant;
use crate::models::processor::Processor;

use super::*;

pub struct Declarant<'a> {
    id: Uuid,
    name: String,
    location: Option<&'a Location>,
    declarations: Vec<Declaration>,
}

impl<'a> Declarant<'a> {
    pub async fn new(id: Uuid, name: &str) -> Declarant<'a> {
        Self {
            id,
            name: name.to_string(),
            location: None,
            declarations: vec![],
        }
    }
}

///
/// We Hide Business Logic behind seperate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs and cut out any logic.
///
pub mod logic {
    pub use super::super::Participant;
    use super::*;
    pub trait Logic: Participant {}

    impl<'a> Participant for Declarant<'a> {
        /// Gets Declaration Copy and fills local Declaration, if any
        /// If there is no such Declaration, it will be added to the list
        async fn fill_declaration(&mut self, declaration: &Declaration) {
            let id = declaration.id().await;
            for decl in &mut self.declarations {
                if decl.id().await == id {
                    *decl = declaration.clone();
                    return;
                }
            }
            self.declarations.push(declaration.clone());
        }

        /// Acceptes Uuid, returns Declaration Reference
        async fn get_declaration(&self, id: Uuid) -> Option<&Declaration> {
            stream::iter(&self.declarations)
                .filter_map(async move |decl| {
                    if decl.id().await == id {
                        Some(decl)
                    } else {
                        None
                    }
                })
                .collect::<Vec<&Declaration>>()
                .await
                .pop()
        }

        async fn send_docs(&self, proc: &Processor, id: Uuid) -> Result<(), Box<dyn Error>> {
            let declaration = self
                .get_declaration(id)
                .await
                .ok_or(DErr::DeclarationNotFound(id))?;
            proc.send_declaration(declaration).await
        }
    }
    impl<'a> Declarant<'a> {}
}

/// Boilerplate
impl<'a> Declarant<'a> {
    getter_ref!( { async } id: &Uuid, { async } name: &str, { async } declarations: &[Declaration]);
    getter_mut!( { async } id: &mut Uuid, { async } name: &mut String, { async } declarations: &mut [Declaration]);
    setter!( { async } id: Uuid, { async } name: &str, { async } declarations: Vec<Declaration>);
    getter!( { async } id: Uuid);
}
