//! Declaration, main transferable object in the system

use uuid::Uuid;

use crate::prelude::*;

#[derive(Clone, Default, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Declaration {
    id: Uuid,
    product_name: String,
    product_code: String,
    product_price: f64,
    product_quantity: i64,
    ptoduct_weight: f64,
    product_description: String,
    transport_type: String,
    transport_id: i64,
    transport_name: String,
    sender_name: String,
    receiver_name: String,
    destination: String,
    departure: String,
    status: String,
    created_at: String,
    updated_at: String,
}

mod logic {
    use super::*;

    impl Declaration {
        pub async fn new() -> Self {
            Self {
                id: Uuid::new_v4(),
                product_name: "".to_string(),
                product_code: "".to_string(),
                product_price: 0.0,
                product_quantity: 0,
                ptoduct_weight: 0.0,
                product_description: "".to_string(),
                transport_type: "".to_string(),
                transport_id: 0,
                transport_name: "".to_string(),
                sender_name: "".to_string(),
                receiver_name: "".to_string(),
                destination: "".to_string(),
                departure: "".to_string(),
                status: "".to_string(),
                created_at: "".to_string(),
                updated_at: "".to_string(),
            }
        }
    }
}

/// Boilerplate
impl Declaration {
    getter_ref!(
        { async } id: &Uuid,
        { async } product_name: &str,
        { async } product_code: &str,
        { async } product_price: &f64,
        { async } product_quantity: &i64,
        { async } ptoduct_weight: &f64,
        { async } product_description: &str,
        { async } transport_type: &str,
        { async } transport_id: &i64,
        { async } transport_name: &str,
        { async } sender_name: &str,
        { async } receiver_name: &str,
        { async } destination: &str,
        { async } departure: &str,
        { async } status: &str,
        { async } created_at: &str,
        { async } updated_at: &str
    );
    getter_mut!( 
        { async } id: &mut Uuid,
        { async } product_name: &mut String,
        { async } product_code: &mut String,
        { async } product_price: &mut f64,
        { async } product_quantity: &mut i64,
        { async } ptoduct_weight: &mut f64,
        { async } product_description: &mut String,
        { async } transport_type: &mut String,
        { async } transport_id: &mut i64,
        { async } transport_name: &mut String,
        { async } sender_name: &mut String,
        { async } receiver_name: &mut String, 
        { async } destination: &mut String,
        { async } departure: &mut String,
        { async } status: &mut String,
        { async } created_at: &mut String,
        { async } updated_at: &mut String
);
    setter!( 
        { async } id: Uuid,
        { async } product_name: &str,
        { async } product_code: &str,
        { async } product_price: f64,
        { async } product_quantity: i64,
        { async } ptoduct_weight: f64,
        { async } product_description: &str,
        { async } transport_type: &str,
        { async } transport_id: i64,
        { async } transport_name: &str,
        { async } sender_name: &str,
        { async } receiver_name: &str,
        { async } destination: &str,
        { async } departure: &str,
        { async } status: &str,
        { async } created_at: &str,
        { async } updated_at: &str);
    getter!( 
        { async } id: Uuid,
        { async } product_price: f64,
        { async } product_quantity: i64,
        { async } ptoduct_weight: f64,
        { async } transport_id: i64
        );
}


