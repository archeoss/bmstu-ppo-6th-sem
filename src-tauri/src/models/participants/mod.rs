pub mod client;
pub mod declarant;
pub mod representative;
use uuid::Uuid;

use crate::prelude::*;

use super::declaration::{Declaration, DeclarationGeneric};
use super::processor::Processor;

pub trait Participant {
    async fn update_declaration(
        &mut self,
        declaration: &DeclarationGeneric,
    ) -> Result<Option<DeclarationGeneric>, Box<dyn Error>>;
    async fn get_declaration(&self, id: Uuid) -> Option<&DeclarationGeneric>;
    async fn send_docs(&mut self, proc: &mut Processor, id: Uuid) -> Result<(), Box<dyn Error>>;
}
