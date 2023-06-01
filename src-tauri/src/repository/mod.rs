use std::error::Error;
use uuid::Uuid;

mod surreal_structs;
pub mod surrealdb;

pub trait CrudOps<T> {
    async fn get(&self, id: Uuid) -> Result<T, Box<dyn Error>>;
    async fn save(&self, id: Uuid, value: T) -> Result<Option<T>, Box<dyn Error>>;
    async fn delete(&self, id: Uuid) -> Result<T, Box<dyn Error>>;
    async fn delete_all(&self) -> Result<Vec<T>, Box<dyn Error>>;
}

pub trait Repository<E, C>
where
    C: CrudOps<E>,
{
    fn new(connection: C) -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;
    fn connection(&self) -> C;
    async fn get(&self, id: Uuid) -> Result<E, Box<dyn Error>> {
        self.connection().get(id).await
    }
    async fn save(&self, entry: Uuid, entity: E) -> Result<Option<E>, Box<dyn Error>> {
        self.connection().save(entry, entity).await
    }
    async fn delete(&self, id: Uuid) -> Result<E, Box<dyn Error>> {
        self.connection().delete(id).await
    }
    async fn delete_all(&self) -> Result<Vec<E>, Box<dyn Error>> {
        self.connection().delete_all().await
    }
}
