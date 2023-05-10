//! Declaration, main transferable object in the system

use std::any::TypeId;

use chrono::Utc;
use uuid::Uuid;

use crate::prelude::*;

/// Declaration States
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Draft;
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Pending;
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Inspecting;
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Approved;
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Rejected;

pub enum Document {
    Declaration(Declaration),
    Billing(Billing),
    Tax(Tax),
}

#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Declaration<State = Draft> {
    id: Uuid,
    signed_by: Uuid,
    inspected_by: Option<Uuid>,
    product_name: String,
    product_code: String,
    product_price: f64,
    product_quantity: i64,
    product_weight: f64,
    product_description: String,
    transport_type: String,
    transport_name: String,
    sender_name: String,
    receiver_name: String,
    destination: String,
    departure: String,
    //state: std::marker::PhantomData<State>,  // This produces warnings from clippy (State doesnt
    // implement Sync (and Send for that matter). Might be unsafe to transfer between threads?
    // Or might be a false negative.
    // fn() -> State is fine tho.
    state: std::marker::PhantomData<fn() -> State>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

#[derive(Clone, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum DeclarationGeneric {
    Draft(Declaration<Draft>),
    Pending(Declaration<Pending>),
    Inspecting(Declaration<Inspecting>),
    Approved(Declaration<Approved>),
    Rejected(Declaration<Rejected>),
}

pub trait GenericDowncast<'a, T: 'a> {
    fn downcast(&'a self) -> Option<&'a T>;
}

impl GenericDowncast<'_, Declaration<Draft>> for DeclarationGeneric {
    fn downcast(&self) -> Option<&Declaration<Draft>> {
        match self {
            Self::Draft(decl) => Some(decl),
            _ => None,
        }
    }
}

impl GenericDowncast<'_, Declaration<Pending>> for DeclarationGeneric {
    fn downcast(&self) -> Option<&Declaration<Pending>> {
        match self {
            Self::Pending(decl) => Some(decl),
            _ => None,
        }
    }
}

impl GenericDowncast<'_, Declaration<Inspecting>> for DeclarationGeneric {
    fn downcast(&self) -> Option<&Declaration<Inspecting>> {
        match self {
            Self::Inspecting(decl) => Some(decl),
            _ => None,
        }
    }
}

impl GenericDowncast<'_, Declaration<Approved>> for DeclarationGeneric {
    fn downcast(&self) -> Option<&Declaration<Approved>> {
        match self {
            Self::Approved(decl) => Some(decl),
            _ => None,
        }
    }
}

impl GenericDowncast<'_, Declaration<Rejected>> for DeclarationGeneric {
    fn downcast(&self) -> Option<&Declaration<Rejected>> {
        match self {
            Self::Rejected(decl) => Some(decl),
            _ => None,
        }
    }
}

impl Default for DeclarationGeneric {
    fn default() -> Self {
        Self::Draft(Declaration::<Draft>::default())
    }
}

mod logic {
    use super::*;
    use std::convert::From;
    use std::error::Error;

    impl Declaration {
        #[tracing::instrument]
        pub async fn new() -> Self {
            let id = Uuid::new_v4();
            tracing::info!("New Declaration created with id: {}", id);
            Self {
                id,
                ..Default::default()
            }
        }
    }

    impl Declaration<Draft> {
        pub async fn is_filled(&self) -> bool {
            for value in [
                &self.departure,
                &self.product_name,
                &self.product_code,
                &self.product_description,
                &self.transport_type,
                &self.transport_name,
                &self.sender_name,
                &self.receiver_name,
                &self.destination,
                &self.departure,
            ] {
                if *value == String::default() {
                    return false;
                }
            }

            for value in [self.product_price, self.product_weight] {
                if value == f64::default() {
                    return false;
                }
            }

            for value in [self.product_quantity] {
                if value == i64::default() {
                    return false;
                }
            }

            true
        }

        pub async fn validate(&self) -> Result<Declaration<Pending>, Box<dyn Error>> {
            if !self.is_filled().await {
                return Err(Box::new(DErr::DeclarationNotComplete(self.id)));
            }

            Ok(Declaration::<Pending>::from(self.clone()))
        }
    }

    fn copy<T, U>(value: Declaration<U>) -> Declaration<T> {
        Declaration {
            id: value.id,
            signed_by: value.signed_by,
            inspected_by: value.inspected_by,
            product_name: value.product_name,
            product_code: value.product_code,
            product_price: value.product_price,
            product_quantity: value.product_quantity,
            product_weight: value.product_weight,
            product_description: value.product_description,
            transport_type: value.transport_type,
            transport_name: value.transport_name,
            sender_name: value.sender_name,
            receiver_name: value.receiver_name,
            destination: value.destination,
            departure: value.departure,
            state: std::marker::PhantomData,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }

    impl From<Declaration<Draft>> for Declaration<Pending> {
        fn from(value: Declaration<Draft>) -> Self {
            copy(value)
        }
    }

    impl From<Declaration<Pending>> for Declaration<Inspecting> {
        fn from(value: Declaration<Pending>) -> Self {
            copy(value)
        }
    }

    impl From<Declaration<Inspecting>> for Declaration<Pending> {
        fn from(value: Declaration<Inspecting>) -> Self {
            copy(value)
        }
    }

    impl From<Declaration<Inspecting>> for Declaration<Approved> {
        fn from(value: Declaration<Inspecting>) -> Self {
            copy(value)
        }
    }

    impl From<Declaration<Inspecting>> for Declaration<Rejected> {
        fn from(value: Declaration<Inspecting>) -> Self {
            copy(value)
        }
    }

    impl Billing {
        pub async fn new() -> Self {
            Self {
                id: Uuid::new_v4(),
                ..Default::default()
            }
        }
    }
}

/// Boilerplate
impl<State> Declaration<State> {
    pub async fn fields(&self) -> ([String; 9], [f64; 2], [i64; 1]) {
        (
            [
                self.product_name.clone(),
                self.product_code.clone(),
                self.product_description.clone(),
                self.transport_type.clone(),
                self.transport_name.clone(),
                self.sender_name.clone(),
                self.receiver_name.clone(),
                self.destination.clone(),
                self.departure.clone(),
            ],
            [self.product_price, self.product_weight],
            [self.product_quantity],
        )
    }
    getter_ref!(
        { async } id: &Uuid,
        { async } signed_by: &Uuid,
        { async } inspected_by: &Option<Uuid>,
        { async } product_name: &str,
        { async } product_code: &str,
        { async } product_price: &f64,
        { async } product_quantity: &i64,
        { async } product_weight: &f64,
        { async } product_description: &str,
        { async } transport_type: &str,
        { async } transport_name: &str,
        { async } sender_name: &str,
        { async } receiver_name: &str,
        { async } destination: &str,
        { async } departure: &str,
        { async } created_at: &chrono::DateTime<Utc>,
        { async } updated_at: &chrono::DateTime<Utc>
    );
    getter_mut!(
        { async } id: &mut Uuid,
        { async } signed_by: &mut Uuid,
        { async } product_name: &mut String,
        { async } product_code: &mut String,
        { async } product_price: &mut f64,
        { async } product_quantity: &mut i64,
        { async } product_weight: &mut f64,
        { async } product_description: &mut String,
        { async } transport_type: &mut String,
        { async } transport_name: &mut String,
        { async } sender_name: &mut String,
        { async } receiver_name: &mut String,
        { async } destination: &mut String,
        { async } departure: &mut String,
        { async } created_at: &mut chrono::DateTime<Utc>,
        { async } updated_at: &mut chrono::DateTime<Utc>
    );

    setter!(
        { async } id: Uuid,
        { async } signed_by: Uuid,
        { async } inspected_by: Option<Uuid>,
        { async } product_name: &str,
        { async } product_code: &str,
        { async } product_price: f64,
        { async } product_quantity: i64,
        { async } product_weight: f64,
        { async } product_description: &str,
        { async } transport_type: &str,
        { async } transport_name: &str,
        { async } sender_name: &str,
        { async } receiver_name: &str,
        { async } destination: &str,
        { async } departure: &str,
        { async } created_at: chrono::DateTime<Utc>,
        { async } updated_at: chrono::DateTime<Utc>
    );

    getter!(
        { async } id: Uuid,
        { async } signed_by: Uuid,
        { async } inspected_by: Option<Uuid>,
        { async } product_price: f64,
        { async } product_quantity: i64,
        { async } product_weight: f64,
        { async } created_at: chrono::DateTime<Utc>,
        { async } updated_at: chrono::DateTime<Utc>
    );
}

#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Billing {
    id: Uuid,
    declaration_id: Uuid,
    sender_id: Uuid,
    receiver_id: Uuid,
    // status: String,
    created_at: chrono::DateTime<Utc>,
    price: f64,
}

/// Boilerplate
impl Billing {
    getter_ref!(
        { async } id: &Uuid,
        { async } declaration_id: &Uuid,
        { async } sender_id: &Uuid,
        { async } receiver_id: &Uuid,
        // { async } status: &str,
        { async } created_at: &chrono::DateTime<Utc>,
        { async } price: &f64
    );
    getter_mut!(
        { async } id: &mut Uuid,
        { async } declaration_id: &mut Uuid,
        { async } sender_id: &mut Uuid,
        { async } receiver_id: &mut Uuid,
        // { async } status: &mut String,
        { async } created_at: &mut chrono::DateTime<Utc>,
        { async } price: &mut f64
    );
    setter!(
        { async } id: Uuid,
        { async } declaration_id: Uuid,
        { async } sender_id: Uuid,
        { async } receiver_id: Uuid,
        // { async } status: &str,
        { async } created_at: chrono::DateTime<Utc>,
        { async } price: f64
    );
    getter!(
        { async } id: Uuid,
        { async } price: f64
    );
}

#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Tax {
    id: Uuid,
    declaration_id: Uuid,
    inspector_id: Uuid,
    receiver_id: Uuid,
    // status: String,
    created_at: chrono::DateTime<Utc>,
    incorrect_fields: usize,
    price: f64,
}

impl Tax {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            declaration_id: Uuid::new_v4(),
            inspector_id: Uuid::new_v4(),
            receiver_id: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            incorrect_fields: 0,
            price: 0.0,
        }
    }
}

/// Boilerplate
impl Tax {
    getter_ref!(
        { async } id: &Uuid,
        { async } declaration_id: &Uuid,
        { async } inspector_id: &Uuid,
        { async } receiver_id: &Uuid,
        // { async } status: &str,
        { async } created_at: &chrono::DateTime<Utc>,
        { async } incorrect_fields: &usize,
        { async } price: &f64
    );
    getter_mut!(
        { async } id: &mut Uuid,
        { async } declaration_id: &mut Uuid,
        { async } inspector_id: &mut Uuid,
        { async } receiver_id: &mut Uuid,
        // { async } status: &mut String,
        { async } created_at: &mut chrono::DateTime<Utc>,
        { async } incorrect_fields: &mut usize,
        { async } price: &mut f64
    );
    setter!(
        { async } id: Uuid,
        { async } declaration_id: Uuid,
        { async } inspector_id: Uuid,
        { async } receiver_id: Uuid,
        // { async } status: &str,
        { async } created_at: chrono::DateTime<Utc>,
        { async } incorrect_fields: usize,
        { async } price: f64
    );
    getter!(
        { async } id: Uuid,
        { async } incorrect_fields: usize,
        { async } price: f64
    );
}

mod tests {
    use super::*;
    #[tokio::test]
    async fn test_getters() {
        let mut d = super::Declaration::<Draft>::default();
        d.set_id("f6d4f3c4-2b1c-4c27-8e75-7f8b9c9b9a9e".parse().unwrap())
            .await;
        d.set_product_name("product name").await;
        d.set_product_code("product code").await;
        d.set_product_price(1.0).await;
        d.set_product_quantity(2).await;
        d.set_product_weight(3.0).await;
        d.set_product_description("product description").await;
        d.set_transport_type("transport type").await;
        d.set_transport_name("transport name").await;
        d.set_sender_name("sender name").await;
        d.set_receiver_name("receiver name").await;
        d.set_destination("destination").await;
        d.set_departure("departure").await;

        assert_eq!(d.product_name_ref().await, "product name");
        assert_eq!(d.product_code_ref().await, "product code");
        assert_eq!(d.product_price().await, 1.0);
        assert_eq!(d.product_quantity().await, 2);
        assert_eq!(d.product_weight().await, 3.0);
        assert_eq!(d.product_description_ref().await, "product description");
        assert_eq!(d.transport_type_ref().await, "transport type");
        assert_eq!(d.transport_name_ref().await, "transport name");
        assert_eq!(d.sender_name_ref().await, "sender name");
        assert_eq!(d.receiver_name_ref().await, "receiver name");
        assert_eq!(d.destination_ref().await, "destination");
        assert_eq!(d.departure_ref().await, "departure");
    }

    #[tokio::test]
    async fn validate() {
        let mut d = super::Declaration::default();
        d.set_id("f6d4f3c4-2b1c-4c27-8e75-7f8b9c9b9a9e".parse().unwrap())
            .await;
        d.set_product_name("product name").await;
        d.set_product_code("product code").await;
        d.set_product_price(1.0).await;
        d.set_product_quantity(2).await;
        d.set_product_weight(3.0).await;
        d.set_product_description("product description").await;
        d.set_transport_type("transport type").await;
        d.set_transport_name("transport name").await;
        d.set_sender_name("sender name").await;
        d.set_receiver_name("receiver name").await;
        d.set_destination("destination").await;
        d.set_departure("departure").await;
        d.set_created_at(chrono::Utc::now()).await;
        d.set_updated_at(chrono::Utc::now()).await;

        let mut d_pending = super::Declaration::<Pending>::default();
        d_pending
            .set_id("f6d4f3c4-2b1c-4c27-8e75-7f8b9c9b9a9e".parse().unwrap())
            .await;
        d_pending.set_product_name("product name").await;
        d_pending.set_product_code("product code").await;
        d_pending.set_product_price(1.0).await;
        d_pending.set_product_quantity(2).await;
        d_pending.set_product_weight(3.0).await;
        d_pending
            .set_product_description("product description")
            .await;
        d_pending.set_transport_type("transport type").await;
        d_pending.set_transport_name("transport name").await;
        d_pending.set_sender_name("sender name").await;
        d_pending.set_receiver_name("receiver name").await;
        d_pending.set_destination("destination").await;
        d_pending.set_departure("departure").await;
        d_pending.set_created_at(d.created_at().await).await;
        d_pending.set_updated_at(d.updated_at().await).await;

        let d_validated = d.validate().await;
        assert!(d_validated.is_ok());
        assert_eq!(d_pending, d_validated.unwrap());
    }
}
