use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    DeclarationNotFound(Uuid),
    InvalidField { field: String, value: String },
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeclarationNotFound(id) => {
                write!(f, "The declaration with given uuid not found. Uuid = {id}")
            }
            Self::InvalidField { field, value } => {
                write!(f, "Field {field} has invalid value: {value}")
            }
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
