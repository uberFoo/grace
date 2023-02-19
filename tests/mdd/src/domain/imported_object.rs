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

    use crate::domain::sarzak::store::ObjectStore as SarzakStore;
    use crate::domain::sarzak::types::Object;

    use crate::domain::isa::store::ObjectStore as IsaStore;
    use crate::domain::isa::types::SubtypeA;
    use crate::domain::isa::types::SuperT;

    #[test]
    fn test() {
        let mut store = ObjectStore::new();
        let mut sarzak_store = SarzakStore::new();
        let mut isa_store = IsaStore::new();

        let obj = Object::new(
            "Just a simple object".to_owned(),
            "SO".to_owned(),
            "Simple Object".to_owned(),
            &mut sarzak_store,
        );
        let st = SuperT::SubtypeA(SubtypeA::new("foo".to_owned(), &mut isa_store).id);

        let _ao = AnotherObject::new(&obj, &st, &mut store);
    }
}