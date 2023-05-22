use std::error::Error;
use uuid::Uuid;

mod surrealdb;
// #[async_trait]
pub trait Repository<E> {
    async fn get(&self, id: Uuid) -> Result<E, Box<dyn Error>>;
    async fn get_all(&self) -> Result<Vec<E>, Box<dyn Error>>;
    async fn save(&self, entity: E, entry: &str) -> Result<(), Box<dyn Error>>;
    async fn delete(&self, id: Uuid) -> Result<E, Box<dyn Error>>;
}
