use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    models::{
        declaration::{Approved, Declaration, DeclarationGeneric, Document, Draft, Rejected},
        misc::location::Location,
    },
    prelude::*,
};

use futures::stream;
use futures::StreamExt;

use crate::errors::declaration::Err as DErr;
use crate::models::participants::Participant;
use crate::models::processor::Processor;

use super::*;

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Declarant {
    id: Uuid,
    name: String,
    location: Option<Location>,
    declarations: HashMap<Uuid, DeclarationGeneric>,
}

impl Declarant {
    #[tracing::instrument]
    pub async fn new(name: &str) -> Self {
        let id = Uuid::new_v4();
        tracing::debug!("Creating Declarant: {:?}", id);
        Self::load(id, name).await
    }

    pub async fn load(id: Uuid, name: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            ..Default::default()
        }
    }
}

///
/// We Hide Business Logic behind separate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs (or just strip it out of said logic).
///
/// Import Logic: use ``<path>::<struct>::logic::*;``
///
pub mod logic {
    use crate::models::{
        declaration::DeclarationGeneric, processor::logic::Logic as ProcessorLogic,
    };

    pub use super::super::Participant;
    use super::*;
    pub trait Logic: Participant {}

    impl Participant for Declarant {
        /// Gets Declaration Copy and fills local Declaration, if any
        /// If there is no such Declaration, it will be added to the list
        ///
        /// Like a HashMap, returns old value, if Declaration with same UUID was updated
        /// None otherwise
        #[tracing::instrument]
        async fn update_declaration(
            &mut self,
            declaration: &DeclarationGeneric,
        ) -> Result<Option<DeclarationGeneric>, Box<dyn Error>> {
            match declaration {
                DeclarationGeneric::Draft(decl) => {
                    tracing::debug!("Updating Declaration: {:?}", decl);
                    Ok(self
                        .declarations
                        .insert(decl.id().await, declaration.clone()))
                }
                DeclarationGeneric::Approved(decl) => {
                    tracing::debug!("Updating Declaration: {:?}", decl);
                    Ok(self
                        .declarations
                        .insert(decl.id().await, declaration.clone()))
                }
                DeclarationGeneric::Rejected(decl) => {
                    tracing::debug!("Updating Declaration: {:?}", decl);
                    Ok(self
                        .declarations
                        .insert(decl.id().await, declaration.clone()))
                }
                DeclarationGeneric::Pending(decl) => {
                    tracing::debug!("Updating Declaration: {:?}", decl);
                    Ok(self
                        .declarations
                        .insert(decl.id().await, declaration.clone()))
                }
                DeclarationGeneric::Inspecting(decl) => {
                    tracing::debug!(
                        "Updating Declaration: {:?}, Declarant UUID: {}",
                        decl,
                        self.id
                    );
                    Ok(self
                        .declarations
                        .insert(decl.id().await, declaration.clone()))
                }
            }
        }

        /// Acceptes Uuid, returns Declaration Reference
        async fn get_declaration(&self, id: Uuid) -> Option<&DeclarationGeneric> {
            self.declarations.get(&id)
        }

        #[tracing::instrument]
        async fn send_docs(
            &mut self,
            proc: &mut Processor,
            id: Uuid,
        ) -> Result<(), Box<dyn Error>> {
            tracing::debug!("Sending Docs for Declaration: UUID={:?}", id);
            let declaration = self.get_declaration(id).await;
            if declaration.is_none() {
                tracing::error!(
                    "Declaration not found: Declarant UUID={}, declaration UUID={:?}",
                    self.id,
                    id
                );
            }
            let declaration = declaration.ok_or(DErr::DeclarationNotFound(id))?;
            if let DeclarationGeneric::Draft(decl) = declaration {
                let decl = decl.validate().await?;
                proc.process_declaration(&decl).await?;
                self.declarations
                    .insert(decl.id().await, DeclarationGeneric::Pending(decl));
                tracing::info!(
                    "Declaration UUID={} status changed from Draft to Pending",
                    id
                );
                Ok(())
            } else {
                tracing::error!("Declaration UUID={} is not in Draft state", id);
                Err(Box::new(DErr::IncorrectState(id, "Not Draft".to_string())))
            }
        }
    }
    impl Declarant {}
}

/// Boilerplate
impl Declarant {
    getter_ref!( { async } id: &Uuid, { async } name: &str, { async } declarations: &HashMap<Uuid, DeclarationGeneric>);
    getter_mut!( { async } id: &mut Uuid, { async } name: &mut String, { async } declarations: &mut HashMap<Uuid, DeclarationGeneric>);
    setter!( { async } id: Uuid, { async } name: &str, { async } declarations: HashMap<Uuid, DeclarationGeneric>);
    getter!( { async } id: Uuid);
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::models::{
        customs::{logic::Logic, Customs},
        declaration::{Declaration, DeclarationGeneric, Draft, Inspecting, Pending},
        misc::location::Location,
        processor::{logic::Logic as PLogic, Processor},
    };

    use super::{logic::*, Declarant};

    async fn generate_declaration() -> Declaration<Draft> {
        let mut declaration = Declaration::<Draft>::default();
        declaration
            .set_sender_name("John")
            .await
            .set_id(Uuid::new_v4())
            .await
            .set_departure("Test")
            .await
            .set_product_name("Test")
            .await
            .set_product_code("Test")
            .await
            .set_product_price(1.0)
            .await
            .set_product_quantity(1)
            .await
            .set_product_weight(1.0)
            .await
            .set_product_description("Test")
            .await
            .set_transport_type("Test")
            .await
            .set_transport_name("Test")
            .await
            .set_receiver_name("Test")
            .await
            .set_destination("Test")
            .await
            .set_created_at(chrono::Utc::now())
            .await
            .set_updated_at(chrono::Utc::now())
            .await;

        declaration
    }

    #[tokio::test]
    async fn update_declaration() {
        let mut declarant = Declarant::new("John").await;
        let mut declaration = Declaration::<Draft>::default();
        let id = declaration.id_ref().await.clone();
        declaration.set_receiver_name("Steve").await;
        declarant
            .update_declaration(&DeclarationGeneric::Draft(declaration))
            .await;

        assert_eq!(declarant.declarations.len(), 1);
        let mut name = String::new();
        if let DeclarationGeneric::Draft(decl) = &declarant.declarations[&id] {
            name = decl.receiver_name_ref().await.to_string();
        }
        assert_eq!(name, "Steve");

        let mut declaration_new = Declaration::<Pending>::default();
        declaration_new
            .set_receiver_name("Jane")
            .await
            .set_id(id)
            .await;
        declarant
            .update_declaration(&DeclarationGeneric::Pending(declaration_new))
            .await;

        assert_eq!(declarant.declarations.len(), 1);
        let mut name = String::new();
        if let DeclarationGeneric::Pending(decl) = &declarant.declarations[&id] {
            name = decl.receiver_name_ref().await.to_string();
        }
        assert_eq!(name, "Jane");
    }

    #[tokio::test]
    async fn get_declaration() {
        let mut declarant = Declarant::new("John").await;
        let mut declaration = Declaration::<Draft>::default();
        let id = declaration.id_ref().await.clone();
        declaration.set_receiver_name("Steve").await;
        declarant
            .update_declaration(&DeclarationGeneric::Draft(declaration.clone()))
            .await;

        assert_eq!(declarant.declarations.len(), 1);
        let mut name = String::new();
        if let DeclarationGeneric::Draft(decl) = &declarant.declarations[&id] {
            name = decl.receiver_name_ref().await.to_string();
        }
        assert_eq!(name, "Steve");

        let declaration_get = declarant.get_declaration(id).await;
        assert!(declaration_get.is_some());
        assert_eq!(
            *declaration_get.unwrap(),
            DeclarationGeneric::Draft(declaration)
        );

        // same tests for other cases
        let mut declaration_new = Declaration::<Pending>::default();
        declaration_new
            .set_receiver_name("Jane")
            .await
            .set_id(id)
            .await;
        declarant
            .update_declaration(&DeclarationGeneric::Pending(declaration_new.clone()))
            .await;

        assert_eq!(declarant.declarations.len(), 1);

        let declaration_get = declarant.get_declaration(id).await;
        assert!(declaration_get.is_some());
        assert_eq!(
            *declaration_get.unwrap(),
            DeclarationGeneric::Pending(declaration_new)
        );

        // same tests for other cases
        let mut declaration_new = Declaration::<Inspecting>::default();
        declaration_new
            .set_receiver_name("Jane")
            .await
            .set_id(id)
            .await;
        declarant
            .update_declaration(&DeclarationGeneric::Inspecting(declaration_new.clone()))
            .await;

        assert_eq!(declarant.declarations.len(), 1);

        let declaration_get = declarant.get_declaration(id).await;
        assert!(declaration_get.is_some());
        assert_eq!(
            *declaration_get.unwrap(),
            DeclarationGeneric::Inspecting(declaration_new)
        );
    }

    #[tokio::test]
    async fn send_docs() {
        let mut declarant = Declarant::new("John").await;
        let mut declaration = generate_declaration().await;
        let id = declaration.id_ref().await.clone();
        declarant
            .update_declaration(&DeclarationGeneric::Draft(declaration.clone()))
            .await;

        assert_eq!(declarant.declarations.len(), 1);
        let mut name = String::new();
        if let DeclarationGeneric::Draft(decl) = &declarant.declarations[&id] {
            name = decl.receiver_name_ref().await.to_string();
        }
        assert_eq!(name, "Test");

        let mut processor = Processor::new().await;
        let cumstoms = Customs::new("Test", &Location::default()).await;
        processor.connect(cumstoms.clone()).await;
        declarant.send_docs(&mut processor, id).await.unwrap();

        assert_eq!(declarant.declarations.len(), 1);
        let mut name = String::new();
        let cumstoms = &processor.customs_ref().await[cumstoms.id_ref().await];
        assert_eq!(
            cumstoms
                .get_declaration(declaration.id_ref().await)
                .await
                .unwrap(),
            DeclarationGeneric::Pending(declaration.into())
        );
    }
}
