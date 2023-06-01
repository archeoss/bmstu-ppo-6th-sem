use std::collections::HashMap;

use uuid::Uuid;

use crate::{prelude::*, utils::HasId};

use super::{customs::Customs, declaration::Declaration};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Processor {
    customs: HashMap<Uuid, Customs>,
}

impl Processor {
    pub async fn new() -> Processor {
        Processor {
            customs: HashMap::default(),
        }
    }
}

///
/// We Hide Business Logic behind seperate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs and cut out any logic.
///
pub mod logic {
    use futures::{stream, StreamExt};
    use rand::Rng;
    use uuid::Uuid;

    use crate::models::{
        customs::{logic::Logic as CustomsLogic, Customs},
        declaration::{Draft, Pending},
    };

    use super::super::declaration::DeclarationGeneric;
    use super::*;
    use crate::errors::processor::Err as PErr;
    use std::error::Error;
    pub trait Logic {
        async fn connect(&mut self, customs: Customs) -> Result<Option<Customs>, Box<dyn Error>>;
        async fn process_declaration(
            &mut self,
            decl: &Declaration<Pending>,
        ) -> Result<Option<Declaration<Pending>>, Box<dyn Error>>;
        async fn find_declaration(&self, id: Uuid) -> Option<DeclarationGeneric>;
    }

    impl Logic for super::Processor {
        #[tracing::instrument]
        async fn connect(&mut self, customs: Customs) -> Result<Option<Customs>, Box<dyn Error>> {
            let id = customs.id().await;
            let customs = self.customs.insert(id, customs);
            if customs.is_some() {
                tracing::warn!("Customs with id {} already exists. Overwriting.", id);
            }

            Ok(customs)
        }

        /// Send declaration to customs.
        #[tracing::instrument]
        async fn process_declaration(
            &mut self,
            decl: &Declaration<Pending>,
        ) -> Result<Option<Declaration<Pending>>, Box<dyn Error>> {
            let customs = self.pick_customs_mut(&decl).await?;
            tracing::info!(
                "Sending declaration {} to customs {}",
                decl.id().await,
                customs.id()
            );

            customs.update_decl(decl.clone()).await
        }

        #[tracing::instrument]
        async fn find_declaration(&self, id: Uuid) -> Option<DeclarationGeneric> {
            tracing::info!("Requested declaration {}", id);
            let decl = stream::iter(self.customs.values())
                .filter_map(async move |customs| customs.get_declaration(&id).await)
                .collect::<Vec<DeclarationGeneric>>()
                .await
                .pop();
            if decl.is_none() {
                tracing::warn!("Declaration {} not found", id);
            }

            decl
        }
    }

    /// Private methods
    impl super::Processor {
        /// Pick a random customs channel. Temporary solution. Replace with a better one.
        async fn pick_customs_mut(
            &mut self,
            decl: &Declaration<Pending>,
        ) -> Result<&mut Customs, Box<dyn Error>> {
            let index = &self.pick_customs_index(decl).await.clone();
            self.customs
                .get_mut(index)
                .ok_or(Box::new(PErr::CannotBorrowCustoms(*index)))
        }

        /// Pick a random customs channel. Temporary solution. Replace with a better one.
        async fn pick_customs(&self, decl: &Declaration<Pending>) -> &Customs {
            &self.customs[self.pick_customs_index(decl).await]
        }

        /// Pick a random customs channel. Temporary solution. Replace with a better one.
        async fn pick_customs_index(&self, _decl: &Declaration<Pending>) -> &Uuid {
            let mut rng = rand::thread_rng();
            // rng.gen_range(0..self.customs.len());
            self.customs
                .keys()
                .into_iter()
                .nth(rng.gen_range(0..self.customs.len()))
                .unwrap() // O(N), but who cares?
        }
    }
}

/// Boilerplate
impl Processor {
    setter!( { async } customs: HashMap<Uuid, Customs>);
    getter_ref!( { async } customs: &HashMap<Uuid, Customs>);
    getter_mut!( { async } customs: &mut HashMap<Uuid, Customs>);
}

#[cfg(test)]
mod tests {
    use crate::models::{
        declaration::{Draft, GenericDowncast, Pending},
        misc::location::Location,
    };

    use super::{logic::*, *};
    #[tokio::test]
    async fn connect() {
        let mut proc = Processor::new().await;
        let location = Location::default();
        let customs = Customs::new("Moscow", &location).await;
        proc.connect(customs.clone()).await.unwrap();

        assert_eq!(proc.customs.len(), 1);
        assert_eq!(proc.customs[customs.id_ref().await], customs);
    }

    #[tokio::test]
    async fn process_declaration() {
        let mut proc = Processor::new().await;
        let location = Location::default();
        let customs = Customs::new("Moscow", &location).await;
        proc.connect(customs.clone()).await.unwrap();
        let mut decl = Declaration::<Draft>::default();
        decl.set_receiver_name("TEST").await;

        assert_eq!(proc.customs_ref().await.len(), 1);
        assert_eq!(proc.customs[customs.id_ref().await], customs);
    }

    #[tokio::test]
    async fn find_declaration() {
        let mut proc = Processor::new().await;
        let location = Location::default();
        let customs = Customs::new("Moscow", &location).await;
        proc.connect(customs.clone()).await.unwrap();
        let mut decl = Declaration::<Pending>::default();
        decl.set_receiver_name("TEST").await;
        proc.process_declaration(&decl).await.unwrap();
        let decl = proc.find_declaration(decl.id().await).await.unwrap();
        dbg!(&decl);
        let decl: &Declaration<Pending> = decl.downcast().unwrap();

        assert_eq!(decl.receiver_name_ref().await, "TEST");
    }
}
