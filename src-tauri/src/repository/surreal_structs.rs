use std::fmt::Debug;

use crate::errors::declaration::Err as DErr;
use crate::models::declaration::{
    Approved, Declaration, DeclarationGeneric, Draft, Inspecting, Pending, Rejected,
};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;
use surrealdb::sql::Thing;
use uuid::Uuid;
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub(super) struct SurrealDeclarant {
    pub name: String,
    pub location: Thing,
    pub declarations: Vec<Uuid>,
}

#[derive(Clone, Default, Eq, PartialEq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct SurrealLocation {
    pub country: String,
    pub region: String,
    pub city: String,
    #[serde(with = "ts_seconds")]
    pub timezone: DateTime<Utc>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct SurrealDeclaration {
    pub signed_by: Thing,
    pub inspected_by: Thing,
    pub product_name: String,
    pub product_code: String,
    pub product_price: f64,
    pub product_quantity: i64,
    pub product_weight: f64,
    pub product_description: String,
    pub transport_type: String,
    pub transport_name: String,
    pub sender_name: String,
    pub receiver_name: String,
    pub destination: String,
    pub departure: String,
    pub state: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

impl DeclarationGeneric {
    pub async fn try_from(
        id: Uuid,
        value: SurrealDeclaration,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut res: Declaration<Draft> = Declaration::new().await;
        res.set_id(id).await;
        res.set_signed_by(Uuid::parse_str(&value.signed_by.id.to_raw())?)
            .await;
        res.set_product_name(&value.product_name).await;
        res.set_product_code(&value.product_code).await;
        res.set_product_price(value.product_price).await;
        res.set_product_quantity(value.product_quantity).await;
        res.set_product_weight(value.product_weight).await;
        res.set_product_description(&value.product_description)
            .await;
        res.set_transport_type(&value.transport_type).await;
        res.set_transport_name(&value.transport_name).await;
        res.set_sender_name(&value.sender_name).await;
        res.set_receiver_name(&value.receiver_name).await;
        res.set_destination(&value.destination).await;
        res.set_receiver_name(&value.receiver_name).await;
        res.set_created_at(value.created_at).await;
        res.set_updated_at(value.updated_at).await;

        let state = value.state;
        match state {
            state if state == std::any::type_name::<Draft>().to_lowercase() => Ok(Self::Draft(res)),
            state if state == std::any::type_name::<Pending>().to_lowercase() => {
                Ok(Self::Pending(res.into()))
            }
            state if state == std::any::type_name::<Inspecting>().to_lowercase() => {
                Ok(Self::Inspecting(
                    (|| -> Declaration<Pending> { res.into() }()).into(), // On Runtime it will
                                                                          // be optimised right
                                                                          // away
                ))
            }
            state if state == std::any::type_name::<Approved>().to_lowercase() => {
                Ok(Self::Approved(
                    (|| -> Declaration<Inspecting> {
                        || -> Declaration<Pending> { res.into() }().into()
                    }())
                    .into(),
                ))
            }
            state if state == std::any::type_name::<Rejected>().to_lowercase() => {
                Ok(Self::Rejected(
                    (|| -> Declaration<Inspecting> {
                        || -> Declaration<Pending> { res.into() }().into()
                    }())
                    .into(),
                ))
            }
            _ => Err(DErr::IncorrectState(res.id().await, state).into()),
        }
    }
}
