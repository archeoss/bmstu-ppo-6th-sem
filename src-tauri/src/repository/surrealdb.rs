use std::{collections::HashMap, dbg, error::Error, fmt::Debug, pin::Pin, println};

use futures::Future;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use surrealdb::{
    engine::remote::ws::Client,
    sql::{Id, Thing},
    Surreal,
};
use uuid::Uuid;

use crate::models::{customs::inspector::Inspector, participants::declarant::Declarant};

use super::Repository;

#[derive(Debug, Serialize, Deserialize)]
struct DeclarantDB {
    // id: Thing,
    name: String,
    location: Thing,
    // location: Uuid,
    declarations: Vec<Uuid>,
}

type DeclarantPin = Pin<Box<dyn Future<Output = DeclarantDB>>>;

impl From<Declarant> for DeclarantPin {
    fn from(value: Declarant) -> Self {
        Box::pin(async move {
            let id = match value.location_ref().await {
                None => Uuid::default(),
                Some(location) => location.id().await,
            };
            let id = Uuid::new_v4();
            let keys = value.declarations_ref().await.keys().copied().collect();
            DeclarantDB {
                // id: Thing {
                //     tb: "declarant".to_string(),
                //     id: surrealdb::sql::Id::String(value.id().await.to_string()),
                // },
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

pub struct RepositoryImpl<E> {
    connection: Surreal<Client>,
    _phantom: std::marker::PhantomData<E>,
}
//
/// General Implementation of the Repository trait
/// Can't be implemented due to async
/// It's either full generalized functions like below
/// Or specialization for all of types
// default impl<E: Serialize + Debug + DeserializeOwned + Send + Sync> Repository<E> for RepositoryImpl<E> {
//     async fn get(&self, id: Uuid) -> Result<E, Box<dyn Error>> {
//         let result: Option<E> = self
//             .connection
//             .select((std::any::type_name::<E>().to_lowercase(), id.to_string()))
//             .await?;
//
//         result.map_or_else(
//             || {
//                 tracing::error!("GET: No entity found. ID: {id}");
//                 Err("No entity found".into())
//             },
//             |entity| {
//                 tracing::info!("GET: {:?}", entity);
//                 Ok(entity)
//             },
//         )
//     }
//
//     async fn get_all(&self) -> Result<Vec<E>, Box<dyn Error>> {
//         Ok(self
//             .connection
//             .select(std::any::type_name::<E>().to_lowercase())
//             .await?)
//     }
//
//     async fn save(&self, entity: E, entry: &str) -> Result<(), Box<dyn Error>> {
//         let res: E = self
//             .connection
//             // .create(
//             .update((std::any::type_name::<E>().to_lowercase(), entry))
//             .content(entity)
//             .await?;
//
//         Ok(())
//     }
//
//     async fn delete(&self, id: Uuid) -> Result<E, Box<dyn Error>> {
//         let result: Option<E> = self
//             .connection
//             .delete((std::any::type_name::<E>().to_lowercase(), id.to_string()))
//             .await?;
//
//         result.map_or_else(
//             || {
//                 tracing::error!("DELETE: No entity found. ID: {id}");
//                 Err("No entity found".into())
//             },
//             |entity| {
//                 tracing::info!("DELETE: {:?}", entity);
//                 Ok(entity)
//             },
//         )
//     }
// }

impl Repository<Declarant> for RepositoryImpl<Declarant> {
    async fn get(&self, id: Uuid) -> Result<Declarant, Box<dyn Error>> {
        let result: Option<DeclarantDB> = self
            .connection
            .select((
                std::any::type_name::<Declarant>().to_lowercase(),
                id.to_string(),
            ))
            .await?;

        match result {
            None => {
                tracing::error!("GET: No entity found. ID: {id}");
                Err("No entity found".into())
            }
            Some(entity) => {
                let mut res = Declarant::new(&entity.name).await;
                // res.set_id(entity.id);
                // res.set_declarations(HashMap::new());
                println!("{:?}", entity.location);
                // tracing::info!("GET: {:?}", entity);
                Ok(res)
            }
        }
    }

    async fn get_all(&self) -> Result<Vec<Declarant>, Box<dyn Error>> {
        Ok(self
            .connection
            .select(std::any::type_name::<Declarant>().to_lowercase())
            .await?)
    }

    async fn save(&self, entity: Declarant, entry: &str) -> Result<(), Box<dyn Error>> {
        let res: DeclarantPin = entity.into();
        let res = dbg!(res.await);
        let res: DeclarantDB = self
            .connection
            // .create(
            .update((std::any::type_name::<Declarant>().to_lowercase(), entry))
            // .update(
            // ("test", entry),
            // )
            .content(res)
            .await?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<Declarant, Box<dyn Error>> {
        let result: Option<Declarant> = self
            .connection
            .delete((
                std::any::type_name::<Declarant>().to_lowercase(),
                id.to_string(),
            ))
            .await?;

        result.map_or_else(
            || {
                tracing::error!("DELETE: No entity found. ID: {id}");
                Err("No entity found".into())
            },
            |entity| {
                tracing::info!("DELETE: {:?}", entity);
                Ok(entity)
            },
        )
    }
}

impl Repository<Inspector> for RepositoryImpl<Inspector> {
    async fn get(&self, id: Uuid) -> Result<Inspector, Box<dyn Error>> {
        todo!()
    }

    async fn get_all(&self) -> Result<Vec<Inspector>, Box<dyn Error>> {
        todo!()
    }

    async fn save(&self, entity: Inspector, entry: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    async fn delete(&self, id: Uuid) -> Result<Inspector, Box<dyn Error>> {
        todo!()
    }
}

mod tests {
    use std::{error::Error, fs::File, io::Read};

    use serde::{Deserialize, Serialize};
    use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

    use crate::models::participants::declarant::{self, Declarant};
    const CREDENTIALS_FILE: &str = "./build/credentials.json";
    #[derive(Serialize, Deserialize)]
    struct Credentials {
        pub username: String,
        pub password: String,
        pub host: String,
        pub port: u16,
        pub ns: String,
        pub db: String,
        pub sc: String,
    }

    fn read_root_credential(filename: &str) -> Result<Credentials, Box<dyn Error>> {
        let credential: Credentials = serde_json::from_str(&read_file(filename)?)?;

        Ok(credential)
    }

    fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(contents)
    }

    #[tokio::test]
    async fn test_get() {
        use super::super::super::repository::Repository;
        use super::RepositoryImpl;
        use crate::models::participants::declarant::Declarant;

        let credentials = read_root_credential(CREDENTIALS_FILE).unwrap();
        let db = Surreal::new::<Ws>(format!("{}:{}", credentials.host, credentials.port))
            .await
            .unwrap();
        let scope = credentials.sc;
        let root = Root {
            username: &credentials.username,
            password: &credentials.password,
        };
        db.signin(root).await.unwrap();
        db.use_ns(credentials.ns)
            .use_db(credentials.db)
            .await
            .unwrap();

        let repository: RepositoryImpl<Declarant> = RepositoryImpl {
            connection: db,
            _phantom: std::marker::PhantomData,
        };

        // let repository: Box<dyn Repository<Declarant>> = Box::from(repository);
        let decl = Declarant::new("Thomas").await;
        repository
            .save(decl.clone(), &decl.id().await.to_string())
            .await
            .unwrap();
        //
        let declarant: Declarant = dbg!(repository.get(decl.id().await).await.unwrap());
        //
        assert_eq!(declarant.name_ref().await, "Thomas");
    }
}
