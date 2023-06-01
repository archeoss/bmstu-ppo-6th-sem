use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    models::{declaration::Billing, misc::location::Location},
    prelude::*,
    utils::HasId,
};

use super::representative::{Service, ServiceRequest};
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Client {
    #[serde(skip)]
    id: Uuid,
    name: String,
    location: Option<Location>,
    requests: HashMap<Uuid, ServiceRequest>,
    billings: HashMap<Uuid, Billing>,
}

impl Client {
    pub async fn new(name: &str) -> Self {
        Self::load(Uuid::new_v4(), name).await
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
    use uuid::Uuid;

    use crate::errors::client::Err as CErr;
    use crate::models::participants::representative::logic::Logic as RLogic;
    use crate::models::participants::representative::ServiceRequest;
    use crate::models::{
        declaration::Billing,
        participants::representative::{Representative, Service},
    };
    use std::error::Error;

    pub trait Logic {
        async fn request_service(
            &mut self,
            repr: &mut Representative,
            description: String,
            service: Service,
        );
        async fn receive_billing(&mut self, billing: Billing) -> Result<(), Box<dyn Error>>;
    }

    impl Logic for super::Client {
        async fn request_service(
            &mut self,
            repr: &mut Representative,
            description: String,
            service: Service,
        ) {
            let service_req = ServiceRequest {
                id: Uuid::new_v4(),
                client: self.id,
                description,
                service,
            };
            self.requests
                .insert(service_req.id.clone(), service_req.clone());
            repr.receive_service(service_req).await;
        }

        async fn receive_billing(&mut self, billing: Billing) -> Result<(), Box<dyn Error>> {
            if self.billings.contains_key(billing.id_ref().await) {
                tracing::warn!(
                    "Billing with given uuid already exists. UUID = {}",
                    billing.id_ref().await
                );
                return Err(Box::new(CErr::BillingExists(billing.id().await.clone())));
            }

            self.billings.insert(billing.id().await.clone(), billing);
            Ok(())
        }
    }
}

/// Boilerplate
impl Client {
    getter_ref!( { async } id: &Uuid, { async } name: &str);
    getter_mut!( { async } id: &mut Uuid, { async } name: &mut String);
    setter!( { async } id: Uuid, { async } name: &str, { async } location: Option<Location>);
    getter!( { async } id: Uuid, { async } location: Option<Location>);
}

mod tests {
    use super::logic::Logic;
    use super::*;
    use crate::models::participants::representative::Representative;

    #[tokio::test]
    async fn request_service() {
        let mut client = Client::new("Test").await;
        let mut repr = Representative::new("Test").await;
        client
            .request_service(&mut repr, "Test".to_string(), Service::Outsoure)
            .await;
        assert_eq!(repr.service_requests_ref().await.len(), 1);
    }

    #[tokio::test]
    async fn receive_billing() {
        let mut client = Client::new("Test").await;
        let mut repr = Representative::new("Test").await;
        client
            .request_service(&mut repr, "Test".to_string(), Service::Outsoure)
            .await;
        let mut service_req =
            repr.service_requests_ref().await[client.requests.keys().next().unwrap()].clone();
        let mut billing = Billing::new().await;
        client.receive_billing(billing).await.unwrap();
        assert_eq!(client.billings.len(), 1);
    }
}

impl HasId for Client {
    fn id(&mut self) -> &mut Uuid {
        &mut self.id
    }
}
