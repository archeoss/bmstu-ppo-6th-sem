use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    SelectNotFound { id: Uuid, table: String },
    TableNotFound(String),
    DeleteNotFound { table: String, id: Uuid },
    // InvalidField {
    //     id: Uuid,
    //     field: String,
    //     value: String,
    // },
    // DeclarationNotComplete(Uuid),
    // IncorrectState(Uuid, String),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TableNotFound(table) => {
                write!(f, "The table with given name not found. Name={table}")
            }
            Self::SelectNotFound { id, table } => {
                write!(
                    f,
                    "Select operation failed. Field {id} was not found in {table}."
                )
            }
            Self::DeleteNotFound { table, id } => {
                write!(
                    f,
                    "Delete operation failed. Field {id} was not found in {table}."
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
