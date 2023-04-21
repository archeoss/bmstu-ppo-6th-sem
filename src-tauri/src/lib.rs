#![feature(async_fn_in_trait, async_closure)]
// Let it be for now
#![allow(dead_code)]
#![allow(clippy::unused_async)]
#![allow(clippy::needless_lifetimes)]

mod errors;
mod macros;
mod models;
mod repository;
mod viewmodels;

pub mod prelude {
    pub use super::macros::*;
    pub use chrono::serde::ts_seconds;
    pub use serde::{Deserialize, Serialize};
    pub use std::error::Error;
    pub use std::sync::Arc;
    pub use std::{cell::RefCell, rc::Rc};
    pub use tokio::sync::Mutex;
}
