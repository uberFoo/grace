//! One to Many Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
//! It contains the following model:
use uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// imported_object
pub const UUID_NS: Uuid = uuid!("f1b24300-39bc-5928-ab66-116fd36296b1");

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sarzak_domain::store::ObjectStore as SarzakStore;
    use crate::sarzak_domain::types::Object;

    #[test]
    fn test() {
        let mut store = ObjectStore::new();
        let mut sarzak_store = SarzakStore::new();

        let obj = Object::new(
            "Just a simple object".to_owned(),
            "SO".to_owned(),
            "Simple Object".to_owned(),
            &mut sarzak_store,
        );
        let _ao = AnotherObject::new(&obj, &mut store);
    }
}
