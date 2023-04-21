mod repositories;

trait Repository<T> {
    async fn get(&self, id: i32) -> Option<T>;
    async fn get_all(&self) -> Vec<T>;
    async fn save(&self, entity: T) -> Result<(), String>;
    async fn delete(&self, id: i32) -> Result<(), String>;
}
