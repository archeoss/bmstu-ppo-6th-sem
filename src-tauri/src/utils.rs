use uuid::Uuid;

pub trait HasId {
    fn id(&mut self) -> &mut Uuid;
}
