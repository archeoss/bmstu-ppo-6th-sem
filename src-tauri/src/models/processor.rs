use tokio::sync::watch::Sender;

use crate::prelude::*;

use super::declaration::Declaration;

#[derive(Debug)]
pub struct Processor {
    customs_channels: Vec<Sender<Declaration>>,
}

///
/// We Hide Business Logic behind seperate module.
/// We do this in order to if we want to turn current Structs
/// into DTO Structs and cut out any logic.
///
mod logic {
    use rand::Rng;

    use super::*;
    use std::error::Error;

    impl super::Processor {
        /// Pick a random customs channel. Temporary solution. Replace with a better one.
        fn pick_customs(&self, _decl: &Declaration) -> &Sender<Declaration> {
            let mut rng = rand::thread_rng();
            let index = rng.gen_range(0..self.customs_channels.len());

            &self.customs_channels[index]
        }

        /// Send declaration to customs.
        pub async fn send_declaration(&self, decl: &Declaration) -> Result<(), Box<dyn Error>> {
            let customs = self.pick_customs(decl);
            Ok(customs.send(decl.clone())?)
        }
    }
}

/// Boilerplate
impl Processor {
    pub fn new() -> Self {
        Self {
            customs_channels: vec![],
        }
    }

    setter!( { async } customs_channels: Vec<Sender<Declaration>>);
    getter_ref!( { async } customs_channels: &[Sender<Declaration>]);
    getter_mut!( { async } customs_channels: &mut[Sender<Declaration>]);
}
