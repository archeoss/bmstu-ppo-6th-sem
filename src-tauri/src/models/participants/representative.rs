use std::{collections::HashMap, sync::RwLock};

use uuid::Uuid;

use crate::{
    models::{
        declaration::{Declaration, DeclarationGeneric},
        misc::location::Location,
    },
    prelude::*,
};

use super::client::Client;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Representative {
    id: Uuid,
    name: String,
    location: Option<Location>,
    declarations: HashMap<Uuid, DeclarationGeneric>,
    service_requests: HashMap<Uuid, ServiceRequest>,
    service_prices: [f64; 3],
    #[serde(skip)]
    clients: HashMap<Uuid, Arc<RwLock<Client>>>,
    brokerage_account: f64,
}

impl Representative {
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
    use futures::stream;
    use futures::StreamExt;
    use uuid::Uuid;

    use super::*;
    use crate::errors::declaration::Err as DErr;
    use crate::errors::representative::Err as RErr;
    use crate::models::declaration::Billing;
    use crate::models::declaration::DeclarationGeneric;
    use crate::models::declaration::Document;
    use crate::models::participants::client::logic::Logic as CLogic;
    use crate::models::participants::Participant;
    use crate::models::processor::logic::Logic as ProcessorLogic;
    use crate::models::processor::Processor;
    pub trait Logic: Participant {
        /// Stub method for billing system.
        /// Just increase brokerage_account value
        async fn receive_pay(&mut self, bill: Billing) -> Result<(), Box<dyn Error>>;
        async fn get_client(&mut self, id: &Uuid) -> Option<&Arc<RwLock<Client>>>;
        async fn request_pay(&mut self, service_id: &Uuid) -> Result<(), Box<dyn Error>>;
        async fn receive_service(&mut self, service: ServiceRequest) -> Result<(), Box<dyn Error>>;
    }

    impl Logic for Representative {
        async fn receive_pay(&mut self, bill: Billing) -> Result<(), Box<dyn Error>> {
            self.brokerage_account += bill.price().await;
            Ok(())
        }

        async fn request_pay(&mut self, service_id: &Uuid) -> Result<(), Box<dyn Error>> {
            let service = self
                .service_requests
                .get(service_id)
                .ok_or_else(|| Box::new(RErr::ServiceNotFound(*service_id)))?
                .clone();
            let (client_id, service) = (service.client, service.service);
            let mut billing = Billing::new().await;

            billing
                .set_receiver_id(self.id)
                .await
                .set_created_at(chrono::Utc::now())
                .await;
            match service {
                Service::CustomsPaperwork => {
                    billing.set_price(self.service_prices[0]).await;
                }
                Service::Consultation => {
                    billing.set_price(self.service_prices[1]).await;
                }
                Service::Outsoure => {
                    billing.set_price(self.service_prices[2]).await;
                }
            }
            let client = self
                .get_client(&client_id)
                .await
                .ok_or_else(|| RErr::ClientNotFound(client_id))?;

            match client.write() {
                Ok(mut w_lock) => {
                    w_lock.receive_billing(billing.clone()).await;
                    Ok(())
                }
                Err(_) => Err(Box::new(RErr::ClientWriteLocked(client_id))),
            }
        }

        async fn get_client(&mut self, id: &Uuid) -> Option<&Arc<RwLock<Client>>> {
            self.clients.get(id)
        }

        async fn receive_service(&mut self, service: ServiceRequest) -> Result<(), Box<dyn Error>> {
            let id = service.id;
            self.service_requests.insert(id, service);
            Ok(())
        }
    }

    impl Participant for Representative {
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
    impl Representative {}
}
/// Boilerplate
impl Representative {
    getter_ref!( { async } id: &Uuid, { async } name: &str, { async } declarations: &HashMap<Uuid, DeclarationGeneric>, { async } service_requests: &HashMap<Uuid, ServiceRequest>, { async } service_prices: &[f64; 3]);
    getter_mut!( { async } id: &mut Uuid, { async } name: &mut String, { async } declarations: &mut HashMap<Uuid, DeclarationGeneric>, { async } service_requests: &mut HashMap<Uuid,ServiceRequest>,  { async } service_prices: &mut [f64; 3]);
    setter!( { async } id: Uuid, { async } name: &str, { async } declarations: HashMap<Uuid, DeclarationGeneric>, { async } service_requests: HashMap<Uuid, ServiceRequest>, { async } service_prices: [f64; 3]);
    getter!( { async } id: Uuid);
}

mod tests {
    use crate::models::declaration::{Billing, Draft, GenericDowncast};
    use crate::models::participants::Participant;

    use super::logic::Logic;
    use super::*;

    #[tokio::test]
    async fn receive_pay() {
        let mut repr = super::Representative::new("Test").await;
        repr.receive_pay(Billing::new().await.set_price(1000.0).await.clone())
            .await
            .unwrap();
        assert!(repr.brokerage_account - 1000.0 < f64::EPSILON);
    }

    #[tokio::test]
    async fn receive_service() {
        let mut repr = super::Representative::new("Test").await;
        repr.receive_service(ServiceRequest {
            id: Uuid::new_v4(),
            client: Uuid::new_v4(),
            description: "Test".to_string(),
            service: Service::Consultation,
        })
        .await
        .unwrap();
        assert_eq!(repr.service_requests.len(), 1);
    }

    #[tokio::test]
    async fn update_declaration() {
        let mut repr = super::Representative::new("Test").await;
        let decl = DeclarationGeneric::Draft(Declaration::new().await);
        repr.update_declaration(&decl).await.unwrap();
        assert_eq!(repr.declarations.len(), 1);
    }

    #[tokio::test]
    async fn get_declaration() {
        let mut repr = super::Representative::new("Test").await;
        let decl = Declaration::new().await;
        let id = decl.id().await;
        let decl = DeclarationGeneric::Draft(decl);
        repr.update_declaration(&decl).await.unwrap();
        assert_eq!(repr.get_declaration(id).await, Some(&decl));
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub id: Uuid,
    pub client: Uuid,
    pub description: String,
    pub service: Service,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Service {
    //Appeal // Won't DO, too much work
    CustomsPaperwork,
    Consultation,
    Outsoure,
}
