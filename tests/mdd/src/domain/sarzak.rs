//! And so we have arrived, finally -- sarzak!
//!
use ::uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// not sure really
pub const UUID_NS: Uuid = uuid!("88cd70e0-c065-5c4b-b1ce-69194ab4d0cb");
