use crate::{
    models::{declaration::Declaration, misc::location::Location},
    prelude::*,
};
struct Representative<'a> {
    id: i64,
    name: String,
    location: Option<&'a Location>,
    declarations: Vec<Declaration>,
    // clients
}

impl<'a> Representative<'a> {
    pub async fn new(id: i64, name: &str) -> Representative<'a> {
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
mod logic {
    use futures::stream;
    use futures::StreamExt;
    use uuid::Uuid;

    use super::*;
    use crate::errors::declaration::Err as DErr;
    use crate::models::participants::Participant;
    use crate::models::processor::Processor;

    impl<'a> Participant for Representative<'a> {
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
    impl<'a> Representative<'a> {}
}
/// Boilerplate
impl<'a> Representative<'a> {
    getter_ref!( { async } id: &i64, { async } name: &str, { async } declarations: &[Declaration]);
    getter_mut!( { async } id: &mut i64, { async } name: &mut String, { async } declarations: &mut [Declaration]);
    setter!( { async } id: i64, { async } name: &str, { async } declarations: Vec<Declaration>);
    getter!( { async } id: i64);
}
