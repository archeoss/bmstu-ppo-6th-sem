use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    DeclarationNotFound(Uuid),
    InvalidField {
        id: Uuid,
        field: String,
        value: String,
    },
    DeclarationNotComplete(Uuid),
    IncorrectState(Uuid, String),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeclarationNotFound(id) => {
                write!(f, "The declaration with given uuid not found. UUUID = {id}")
            }
            Self::InvalidField { id, field, value } => {
                write!(f, "Field {field} has invalid value: {value}. UUID = {id}")
            }
            Self::DeclarationNotComplete(id) => {
                write!(f, "Declaration is not ready to be sent. UUID = {id}")
            }
            Self::IncorrectState(id, state) => {
                write!(
                    f,
                    "Declaration has invalid state. UUID = {id}, State = {state}"
                )
            }
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
