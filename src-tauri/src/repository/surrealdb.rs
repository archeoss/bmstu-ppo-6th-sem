use std::{error::Error, marker::PhantomData, pin::Pin};

pub use chrono::serde::ts_seconds;
use futures::Future;
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};
use uuid::Uuid;

use crate::{
    errors::db::Err,
    models::{
        customs::{inspector::Inspector, operator::Operator, Customs},
        declaration::Declaration,
        misc::location::Location,
        participants::{client, declarant::Declarant, representative::Representative},
    },
    utils::HasId,
};

use super::Repository;
use super::{surreal_structs::*, CrudOps};
type DeclarantPin = Pin<Box<dyn Future<Output = SurrealDeclarant>>>;

impl From<Declarant> for DeclarantPin {
    fn from(value: Declarant) -> Self {
        Box::pin(async move {
            let id = match value.location_ref().await {
                None => Uuid::default(),
                Some(location) => location.id().await,
            };
            let keys = value.declarations_ref().await.keys().copied().collect();
            SurrealDeclarant {
                name: value.name_ref().await.to_string(),
                location: Thing {
                    tb: "location".to_string(),
                    id: Id::from(id.to_string()),
                },
                declarations: keys,
            }
        })
    }
}

pub struct SurrealRepo<E> {
    connection: Surreal<Client>,
    _phantom: std::marker::PhantomData<E>,
}

impl<T: DeserializeOwned + Serialize + Send + Sync + HasId> CrudOps<T> for Surreal<Client> {
    async fn get(&self, id: Uuid) -> Result<T, Box<dyn Error>> {
        let result: Option<T> = self
            .select((
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
                id.to_string(),
            ))
            .await?;

        result.map_or_else(
            || {
                tracing::warn!("GET: field not found");
                Err(Err::SelectNotFound {
                    table: std::any::type_name::<T>()
                        .to_lowercase()
                        .rsplit("::")
                        .next()
                        .unwrap_or(&std::any::type_name::<T>().to_lowercase())
                        .to_string(),
                    id,
                }
                .into())
            },
            |mut res| {
                tracing::info!("GET: success");
                *res.id() = id;
                Ok(res)
            },
        )
    }

    async fn save(&self, id: Uuid, value: T) -> Result<Option<T>, Box<dyn Error>> {
        let result: Option<T> = self
            .update((
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
                id.to_string(),
            ))
            .content(value)
            .await?;

        Ok(result)
    }

    async fn delete(&self, id: Uuid) -> Result<T, Box<dyn Error>> {
        let result: Option<T> = self
            .delete((
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
                id.to_string(),
            ))
            .await?;

        result.map_or_else(
            || {
                tracing::warn!("DELETE: field not found");
                Err(Err::SelectNotFound {
                    table: std::any::type_name::<T>()
                        .to_lowercase()
                        .rsplit("::")
                        .next()
                        .unwrap_or(&std::any::type_name::<T>().to_lowercase())
                        .to_string(),
                    id,
                }
                .into())
            },
            |res| {
                tracing::info!("DELETE: success");
                Ok(res)
            },
        )
    }

    async fn delete_all(&self) -> Result<Vec<T>, Box<dyn Error>> {
        Ok(self
            .delete(
                std::any::type_name::<T>()
                    .to_lowercase()
                    .rsplit("::")
                    .next()
                    .unwrap_or(&std::any::type_name::<T>().to_lowercase()),
            )
            .await?)
    }
}

default impl<T: DeserializeOwned + Serialize + Send + Sync + HasId> Repository<T, Surreal<Client>>
    for SurrealRepo<T>
{
    fn new(connection: Surreal<Client>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            connection,
            _phantom: PhantomData::<T>,
        })
    }

    fn connection(&self) -> Surreal<Client> {
        self.connection.clone()
    }
}

impl Repository<Declarant, Surreal<Client>> for SurrealRepo<Declarant> {}
impl<T> Repository<Declaration<T>, Surreal<Client>> for SurrealRepo<Declaration<T>> {}
impl Repository<Representative, Surreal<Client>> for SurrealRepo<Representative> {}
impl Repository<client::Client, Surreal<Client>> for SurrealRepo<client::Client> {}
impl Repository<Customs, Surreal<Client>> for SurrealRepo<Customs> {}
impl Repository<Location, Surreal<Client>> for SurrealRepo<Location> {}
impl Repository<Inspector, Surreal<Client>> for SurrealRepo<Inspector> {}
impl Repository<Operator, Surreal<Client>> for SurrealRepo<Operator> {}
