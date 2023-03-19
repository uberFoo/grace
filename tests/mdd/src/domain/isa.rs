//! Isa Relationship Domain
//!
//! This file was generated by: `sarzak new "Isa Relationship"`.
use uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// Isa Relationship
pub const UUID_NS: Uuid = uuid!("fdd6c56b-f3fb-59ba-b387-31dd1ff762ea");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_r1() {
        let mut store = ObjectStore::new();

        let oh_boy = OhBoy::new("The Wall Street Journal".to_owned(), &mut store);
        let z = SimpleSubtypeA::new_oh_boy(&oh_boy, &mut store);
        let a = SimpleSupertype::new_simple_subtype_a(true, &z, &mut store);
        let b = SimpleSupertype::new_simple_subtype_b(false, &mut store);

        assert_eq!(&a, store.exhume_simple_supertype(&a.id).unwrap());
        assert_eq!(&b, store.exhume_simple_supertype(&b.id).unwrap());

        assert_eq!(&a, z.r1_simple_supertype(&store)[0]);

        let r = Reference::new("this is a reference".to_owned(), &mut store);
        let a = SubtypeA::new("a".to_owned(), &mut store);
        let sa = SuperT::new_subtype_a(&r, &a, &mut store);
        let b = SubtypeB::new(8, &mut store);
        let sb = SuperT::new_subtype_b(&r, &b, &mut store);

        assert_eq!(&a, store.exhume_subtype_a(&a.id).unwrap());
        assert_eq!(&b, store.exhume_subtype_b(&b.id).unwrap());
        assert_eq!(&sa, store.exhume_super_t(&sa.id).unwrap());
        assert_eq!(&sb, store.exhume_super_t(&sb.id).unwrap());

        assert_eq!(&sa, a.r2_super_t(&store)[0]);
        assert_eq!(&sb, b.r2_super_t(&store)[0]);
    }

    #[test]
    fn test_init() {
        let store = ObjectStore::new();

        let mutable = Borrowed::new_mutable();
        let shared = Borrowed::new_shared();

        // Just making sure that they exist in the store. Thsi will panic if they
        // do not.
        store.exhume_ownership(&mutable.id()).unwrap();
        store.exhume_ownership(&shared.id()).unwrap();
    }
}
