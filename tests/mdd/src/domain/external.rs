//! External entity
//!
use ::uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// external
pub const UUID_NS: Uuid = uuid!("587a9b64-23af-5405-ae38-2a49030b8edc");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut _store = ObjectStore::new();
    }
}
