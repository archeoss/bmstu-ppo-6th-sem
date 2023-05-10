use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    BillingExists(Uuid),
    // ClientNotFound(Uuid),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BillingExists(id) => {
                write!(
                    f,
                    "The client already have billing with specified uuid. Create new billing instead. UUID = {id}"
                )
            } // Self::ClientNotFound(id) => {
              //     write!(f, "Client with given uuid not found. UUID = {id}")
              // }
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
