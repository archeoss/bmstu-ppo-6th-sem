use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    models::declaration::{Declaration, Inspecting},
    prelude::*,
};

#[derive(Clone, Default, PartialEq, Debug, Serialize, Deserialize)]
pub struct Inspector {
    id: Uuid,
    name: String,
    rank: String,
    post: String,
    declarations: HashMap<Uuid, Declaration<Inspecting>>,
}

impl Inspector {
    pub async fn new(name: &str, post: &str, rank: &str) -> Self {
        Self::load(Uuid::new_v4(), name, post, rank).await
    }

    pub async fn load(id: Uuid, name: &str, post: &str, rank: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            post: post.to_string(),
            rank: rank.to_string(),
            declarations: HashMap::default(),
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
    use std::error::Error;

    use futures::stream;
    use futures::StreamExt;
    use uuid::Uuid;

    use crate::errors::declaration::Err as PErr;
    use crate::models::customs::CustomsParams;
    use crate::models::declaration::Declaration;
    use crate::models::declaration::DeclarationGeneric;
    use crate::models::declaration::Inspecting;
    use crate::models::declaration::Pending;
    use crate::models::declaration::Tax;
    use crate::models::processor::logic::Logic as PLogic;
    use crate::models::processor::Processor;

    pub trait Logic {
        async fn get_declaration(&self, id: &Uuid) -> Option<&Declaration<Inspecting>>;
        async fn fetch_declaration(&mut self, declaration: Declaration<Pending>);
        async fn update_declaration(
            &mut self,
            declaration: Declaration<Inspecting>,
        ) -> Option<Declaration<Inspecting>>;
        async fn calc_tax(
            &self,
            declaration: &Declaration<Inspecting>,
            declaration_corrected: &Declaration<Inspecting>,
            conf: CustomsParams,
        ) -> Tax;
        async fn remove_declaration(&mut self, id: &Uuid) -> Option<Declaration<Inspecting>>;
        async fn reprocess(
            &mut self,
            processor: &mut Processor,
            id: &Uuid,
        ) -> Result<(), Box<dyn Error>>;
    }

    impl Logic for super::Inspector {
        /// Acceptes Uuid, returns Declaration Reference
        async fn get_declaration(&self, id: &Uuid) -> Option<&Declaration<Inspecting>> {
            self.declarations.get(id)
        }

        async fn remove_declaration(&mut self, id: &Uuid) -> Option<Declaration<Inspecting>> {
            self.declarations.remove(id)
        }

        #[tracing::instrument]
        async fn fetch_declaration(&mut self, declaration: Declaration<Pending>) {
            let mut declaration: Declaration<Inspecting> = declaration.into();
            declaration.set_inspected_by(Some(self.id)).await;
            tracing::info!(
                "Declarartion {} state changed to Inspecting",
                declaration.id().await
            );
            self.declarations
                .insert(declaration.id().await, declaration);
        }

        async fn update_declaration(
            &mut self,
            mut declaration: Declaration<Inspecting>,
        ) -> Option<Declaration<Inspecting>> {
            declaration.set_inspected_by(Some(self.id)).await;
            tracing::info!("Declarartion {} value(-s) changed", declaration.id().await);
            self.declarations
                .insert(declaration.id().await, declaration)
        }

        async fn reprocess(
            &mut self,
            processor: &mut Processor,
            id: &Uuid,
        ) -> Result<(), Box<dyn Error>> {
            let declaration = self.remove_declaration(id).await;
            if let Some(declaration) = declaration {
                let declaration: Declaration<Pending> = declaration.into();
                tracing::info!("Reprocessing declaration {}", id);
                processor.process_declaration(&declaration).await?;
                Ok(())
            } else {
                tracing::error!("Declaration {} not found", id);
                Err(Box::new(PErr::DeclarationNotFound(*id)))
            }
        }

        #[tracing::instrument]
        async fn calc_tax(
            &self,
            declaration_old: &Declaration<Inspecting>,
            declaration_corrected: &Declaration<Inspecting>,
            conf: CustomsParams,
        ) -> Tax {
            let fee = conf.fee;
            let fee_per_item = fee
                .calculate_fee(declaration_corrected.product_price().await)
                .await;
            // Check fields of declaration
            // if fields has changed, then calculate tax
            let mut tax = Tax::new();
            tax.set_inspector_id(self.id).await;
            tax.set_declaration_id(declaration_corrected.id().await)
                .await;
            tax.set_receiver_id(declaration_corrected.signed_by().await)
                .await;
            let (mut incorrect_fields, mut price) = (0, 0.0);
            for (old, new) in declaration_old
                .fields()
                .await
                .0
                .iter()
                .zip(declaration_corrected.fields().await.0.iter())
            {
                if old != new {
                    incorrect_fields += 1;
                    price += fee_per_item;
                }
            }

            for (old, new) in declaration_old
                .fields()
                .await
                .1
                .iter()
                .zip(declaration_corrected.fields().await.1.iter())
            {
                if (old - new).abs() > f64::EPSILON {
                    incorrect_fields += 1;
                    price += fee_per_item;
                }
            }

            for (old, new) in declaration_old
                .fields()
                .await
                .2
                .iter()
                .zip(declaration_corrected.fields().await.2.iter())
            {
                if old != new {
                    incorrect_fields += 1;
                    price += fee_per_item;
                }
            }
            tax.set_incorrect_fields(incorrect_fields)
                .await
                .set_price(price)
                .await;
            tracing::info!("Tax calculated: {:?}", tax);

            tax
        }
    }
}

/// Boilerplate
impl Inspector {
    getter_ref!( { async } id: &Uuid, { async } name: &str, { async } rank: &str, { async } post: &str, { async } declarations: &HashMap<Uuid, Declaration<Inspecting>>);
    getter_mut!(  { async } id: &mut Uuid, { async } name: &mut String, { async } rank: &mut String, { async } post: &mut String, { async } declarations: &mut HashMap<Uuid, Declaration<Inspecting>>);
    setter!( { async } id: Uuid, { async } name: &str, { async } rank: &str, { async } post: &str, { async } declarations: HashMap<Uuid, Declaration<Inspecting>>);
    getter!( { async } id: Uuid);
}

mod tests {
    use super::logic::Logic;
    use super::*;
    use crate::models::{
        customs::{CustomsParams, Fee},
        declaration::{Declaration, Pending},
    };
    #[tokio::test]
    async fn get_declaration() {
        let mut inspector = Inspector::new("Ivan", "Inspector", "Leutenant").await;
        let declaration: Declaration<Pending> = Declaration::new().await.into();
        let id = declaration.id().await;
        inspector.fetch_declaration(declaration).await;
        let declaration = inspector.get_declaration(&id).await;
        assert!(declaration.is_some());
    }

    #[tokio::test]
    async fn remove_declaration() {
        let mut inspector = Inspector::new("Ivan", "Inspector", "Leutenant").await;
        let declaration: Declaration<Pending> = Declaration::new().await.into();
        let id = declaration.id().await;
        inspector.fetch_declaration(declaration).await;
        let declaration = inspector.remove_declaration(&id).await;
        assert!(declaration.is_some());
    }

    #[tokio::test]
    async fn calc_tax() {
        let declaration_old: Declaration<Pending> = Declaration::new().await.into();
        let declaration_corrected: Declaration<Pending> = Declaration::new().await.into();
        let mut declaration_old: Declaration<Inspecting> = declaration_old.into();
        let mut declaration_corrected: Declaration<Inspecting> = declaration_corrected.into();
        let inspector = Inspector::new("Ivan", "Inspector", "Leutenant").await;
        let mut customs_params = CustomsParams::default();
        customs_params.fee = Fee::Flat(10.0);
        let tax = inspector
            .calc_tax(
                &declaration_old,
                &declaration_corrected,
                customs_params.clone(),
            )
            .await;
        assert_eq!(tax.incorrect_fields().await, 0);
        assert!(tax.price().await < f64::EPSILON);
        declaration_old
            .set_product_price(100.0)
            .await
            .set_product_name("Apple")
            .await
            .set_product_code("123")
            .await;
        declaration_corrected
            .set_product_price(200.0)
            .await
            .set_product_name("Apple")
            .await
            .set_product_code("234")
            .await;
        let tax = inspector
            .calc_tax(&declaration_old, &declaration_corrected, customs_params)
            .await;
        assert_eq!(tax.incorrect_fields().await, 2);
        assert!((tax.price().await - 20.0).abs() < f64::EPSILON);
    }
}
