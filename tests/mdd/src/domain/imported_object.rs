//! One to Many Domain
//!
//! This file will eventually be generated.
//!
//! The purpose of this domain is to help me develop and test domain code generation.
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

    use crate::domain::sarzak::store::ObjectStore as SarzakStore;
    use crate::domain::sarzak::types::Object;

    use crate::domain::isa::types::SimpleSupertype;

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

        let st = SimpleSupertype::new_simple_subtype_a();
        let _ao = AnotherObject::new(&obj, &st, &mut store);
    }
}
