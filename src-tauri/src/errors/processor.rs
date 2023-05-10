use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use uuid::Uuid;

#[derive(Debug)]
pub enum Err {
    CannotBorrowCustoms(Uuid),
}

impl Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CannotBorrowCustoms(id) => {
                write!(f, "Can't borrow customs mutabaly, probably already borrowed or customs doesn' exist. Uuid = {id}")
            }
        }
    }
}

impl Error for Err {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
