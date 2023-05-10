use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    ServiceNotFound(Uuid),
    ClientNotFound(Uuid),
    ClientWriteLocked(Uuid),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServiceNotFound(id) => {
                write!(
                    f,
                    "The service request with given uuid not found. UUID = {id}"
                )
            }
            Self::ClientNotFound(id) => {
                write!(f, "Client with given uuid not found. UUID = {id}")
            }
            Self::ClientWriteLocked(id) => {
                write!(f, "Can't lock client to write into. UUID = {id}")
            }
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
