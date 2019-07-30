//! A simple key/value store.

pub use crate::error::{KvsError, Result};
pub use kvs::KvStore;

mod error;
mod kvs;
