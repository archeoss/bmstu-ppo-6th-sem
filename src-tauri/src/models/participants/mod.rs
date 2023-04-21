mod client;
pub mod declarant;
mod representative;
use uuid::Uuid;

use crate::prelude::*;

use super::declaration::Declaration;
use super::processor::Processor;

pub trait Participant {
    async fn fill_declaration(&mut self, declaration: &Declaration);
    async fn get_declaration(&self, id: Uuid) -> Option<&Declaration>;
    async fn send_docs(&self, proc: &Processor, id: Uuid) -> Result<(), Box<dyn Error>>;
}
