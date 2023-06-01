#![feature(async_fn_in_trait, async_closure, specialization)]
// Let it be for now
#![allow(dead_code)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_lifetimes)]

pub mod errors;
pub mod macros;
pub mod models;
pub mod repository;
mod utils;
pub mod viewmodels;

pub mod prelude {
    pub use crate::errors::declaration::Err as DErr;
    // pub use crate::errors::::Err as DErr;
    pub use super::macros::*;
    pub use chrono::serde::ts_seconds;
    pub use serde::{Deserialize, Serialize};
    pub use std::error::Error;
    pub use std::sync::Arc;
    pub use std::{cell::RefCell, rc::Rc};
    pub use tokio::sync::Mutex;
}
