use std::collections::HashMap;

use self::{inspector::Inspector, operator::Operator};

use super::{
    declaration::{Declaration, Pending},
    misc::location::Location,
};
use crate::{prelude::*, utils::HasId};
use chrono::naive::NaiveTime;
use uuid::Uuid;
pub mod inspector;
pub mod operator;
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
enum Fee {
    Percentage(f64),
    Flat(f64),
    ProgressiveFlat { border: Vec<f64>, fee: Vec<f64> },
}

impl Default for Fee {
    fn default() -> Self {
        Self::Flat(0.)
    }
}

impl Fee {
    pub async fn calculate_fee(&self, product_price: f64) -> f64 {
        match self {
            Self::Percentage(perc) => perc * product_price,
            Self::Flat(flat_tax) => *flat_tax,
            Self::ProgressiveFlat { border, fee } => {
                let mut calc_fee = 0.0;
                for (&border, &fee) in border.iter().zip(fee.iter()) {
                    if product_price < border {
                        calc_fee = fee;
                    }
                }

                calc_fee
            }
        }
    }
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomsParams {
    fee: Fee,
    banned_import_products: Vec<String>,
    banned_export_products: Vec<String>,
    banned_import_origin: Vec<String>,
    banned_export_origin: Vec<String>,
}

// #[derive(Clone, PartialEq, PartialOrd, Debug)]
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Customs {
    #[serde(skip)]
    id: Uuid,
    work_hours: Option<(NaiveTime, NaiveTime)>,
    name: Option<String>,
    location: Option<Location>,
    competence: Option<String>,
    phone_number: Option<String>,
    email: Option<String>,
    declarations: HashMap<Uuid, Declaration<Pending>>,
    inspectors: HashMap<Uuid, Inspector>,
    operators: HashMap<Uuid, Operator>,
    customs_params: CustomsParams,
}

impl Customs {
    #![allow(clippy::unwrap_used)]
    /// Generate new customs with unique id
    pub async fn new(name: &str, location: &Location) -> Customs {
        Self::load(Uuid::new_v4(), name, location).await
    }

    /// Load customs (from database likely) with predefined UUID
    pub async fn load(id: Uuid, name: &str, location: &Location) -> Customs {
        Self {
            id,
            work_hours: Some((
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(20, 0, 0).unwrap(),
            )),
            name: Some(name.to_string()),
            location: Some(location.clone()),
            ..Default::default()
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
pub mod logic {
    use crate::errors::declaration::Err as DErr;
    use crate::models::declaration::Declaration;
    use crate::models::declaration::{DeclarationGeneric, Document, Pending};
    use futures::stream;
    use futures::StreamExt;
    use std::error::Error;
    use uuid::Uuid;

    pub trait Logic {
        /// Takes declaration copy, updates declarations, if there is any,
        /// otherwise - add it to the pool
        async fn update_decl(
            &mut self,
            decl: Declaration<Pending>,
        ) -> Result<Option<Declaration<Pending>>, Box<dyn Error>>;
        /// Gives declaration reference with provided UUID, if there is any
        async fn get_declaration(&self, id: &Uuid) -> Option<DeclarationGeneric>;
        // async fn receive_docs(&mut self, doc: Document) -> Result<(), Box<dyn Error>>;
        /// Gives declaration reference with provided UUID, if there is any, and deletes it from
        /// the pool
        async fn remove_declaration(&mut self, id: &Uuid) -> Option<Declaration<Pending>>;
    }

    impl Logic for super::Customs {
        async fn update_decl(
            &mut self,
            decl: Declaration<Pending>,
        ) -> Result<Option<Declaration<Pending>>, Box<dyn Error>> {
            let id = decl.id().await;
            tracing::info!("Updating declaration with id: {}", id);
            let old_decl = self.declarations.insert(id, decl);
            if old_decl.is_some() {
                tracing::info!("Declaration with id: {} was updated", id);
            } else {
                tracing::info!("Declaration with id: {} was added", id);
            }

            Ok(old_decl)
        }

        async fn get_declaration(&self, id: &Uuid) -> Option<DeclarationGeneric> {
            if let Some(decl) = self.declarations.get(id) {
                Some(DeclarationGeneric::Pending(decl.clone()))
            } else {
                tracing::warn!("No declaration with id: {}", id);
                None
            }
        }

        async fn remove_declaration(&mut self, id: &Uuid) -> Option<Declaration<Pending>> {
            self.declarations.remove(id)
        }
    }
}

/// Boilerplate
impl Customs {
    getter_ref!(
        { async } id: &Uuid,
        { async } name: &Option<String>,
        { async } competence: &Option<String>,
        { async } phone_number: &Option<String>,
        { async } email: &Option<String>,
        { async } declarations: &HashMap<Uuid, Declaration<Pending>>,
        { async } inspectors: &HashMap<Uuid, Inspector>,
        { async } operators: &HashMap<Uuid, Operator>
    );

    setter!(
        { async } id: Uuid,
        { async } name: Option<String>,
        { async } competence: Option<String>,
        { async } phone_number: Option<String>,
        { async } email: Option<String>,
        { async } declarations: HashMap<Uuid, Declaration<Pending>>,
        { async } inspectors: HashMap<Uuid, Inspector>,
        { async } operators: HashMap<Uuid, Operator>
    );

    getter_mut!(
        { async } name: &mut Option<String>,
        { async } competence: &mut Option<String>,
        { async } phone_number: &mut Option<String>,
        { async } email: &mut Option<String>,
        { async } declarations: &mut HashMap<Uuid, Declaration<Pending>>,
        { async } inspectors: &mut HashMap<Uuid, Inspector>,
        { async } operators: &mut HashMap<Uuid, Operator>
    );

    getter!(
        { async } id: Uuid,
        { async } name: Option<String>,
        { async } competence: Option<String>,
        { async } phone_number: Option<String>,
        { async } email: Option<String>
    );
}

impl HasId for Customs {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
