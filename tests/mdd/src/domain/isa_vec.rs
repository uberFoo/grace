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

        assert_eq!(a, store.exhume_simple_supertype(&a.borrow().id).unwrap());
        assert_eq!(b, store.exhume_simple_supertype(&b.borrow().id).unwrap());

        assert_eq!(a, z.borrow().r1_simple_supertype(&store)[0]);

        let r = Reference::new("this is a reference".to_owned(), &mut store);
        let a = SubtypeA::new("a".to_owned(), &mut store);
        let sa = SuperT::new_subtype_a(&r, &a, &mut store);
        let b = SubtypeB::new(8, &mut store);
        let sb = SuperT::new_subtype_b(&r, &b, &mut store);

        // assert_eq!(&a, store.exhume_subtype_a(&a.id).unwrap());
        assert_eq!(b, store.exhume_subtype_b(&b.borrow().id).unwrap());
        assert_eq!(sa, store.exhume_super_t(&sa.borrow().id).unwrap());
        assert_eq!(sb, store.exhume_super_t(&sb.borrow().id).unwrap());

        assert_eq!(sa, a.borrow().r2_super_t(&store)[0]);
        assert_eq!(sb, b.borrow().r2_super_t(&store)[0]);
    }

    #[test]
    fn test_r3() {
        let mut store = ObjectStore::new();

        let oh_boy = OhBoy::new("The Wall Street Journal".to_owned(), &mut store);
        let z = SimpleSubtypeA::new_oh_boy(&oh_boy, &mut store);
        let henry = Henry::new("Henry".to_owned(), &z, &mut store);

        assert_eq!(z, henry.borrow().r3_simple_subtype_a(&store)[0]);
        assert_eq!(henry, z.borrow().r3_henry(&store)[0]);
    }

    #[test]
    fn test_r4() {
        let mut store = ObjectStore::new();

        let b = SimpleSupertype::new_simple_subtype_b(false, &mut store);
        let baz = Baz::new(1.0, &b, &mut store);

        assert_eq!(b, baz.borrow().r4_simple_supertype(&store)[0]);
        assert_eq!(baz, b.borrow().r4_baz(&store)[0]);
    }

    #[test]
    fn test_init() {
        let mut store = ObjectStore::new();

        let mutable = Borrowed::new_mutable(&mut store);
        let shared = Borrowed::new_shared(&mut store);

        // Just making sure that they exist in the store. This will panic if they
        // do not.
        store.exhume_borrowed(&mutable.borrow().id).unwrap();
        store.exhume_borrowed(&shared.borrow().id).unwrap();

        let mutable = Ownership::new_borrowed(&mutable, &mut store);
        let shared = Ownership::new_borrowed(&shared, &mut store);
        let owned = Ownership::new_owned(&mut store);
        store.exhume_ownership(&mutable.borrow().id).unwrap();
        store.exhume_ownership(&shared.borrow().id).unwrap();
        store.exhume_ownership(&owned.borrow().id).unwrap();
    }

    #[test]
    fn test_multi_super_sub() {
        let mut store = ObjectStore::new();

        #[allow(non_snake_case)]
        let Γ = Gamma::new(3.14, &mut store);
        let α = Alpha::new_gamma("α".to_owned(), &Γ, &mut store);
        let β = Beta::new_gamma("β".to_owned(), &Γ, &mut store);

        assert_eq!(α, Γ.borrow().r10_alpha(&store)[0]);
        assert_eq!(β, Γ.borrow().r11_beta(&store)[0]);

        let sf = SuperFoo::new_gamma(&Γ, &mut store);
        let sb = SuperBar::new_gamma(&Γ, &mut store);

        assert_eq!(sf, Γ.borrow().r13_super_foo(&store)[0]);
        assert_eq!(sb, Γ.borrow().r12_super_bar(&store)[0]);
    }
}
