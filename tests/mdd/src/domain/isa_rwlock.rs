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
    use tracy_client::Client;

    #[test]
    fn test_r1() {
        Client::start();

        let mut store = ObjectStore::new();

        let oh_boy = OhBoy::new("The Wall Street Journal".to_owned(), &mut store);
        let z = SimpleSubtypeA::new_oh_boy(&oh_boy, &mut store);
        let a = SimpleSupertype::new_simple_subtype_a(true, &z, &mut store);
        let b = SimpleSupertype::new_simple_subtype_b(false, &mut store);

        assert_eq!(
            &*a.read().unwrap(),
            &*store
                .exhume_simple_supertype(&a.read().unwrap().id)
                .unwrap()
                .read()
                .unwrap()
        );
        assert_eq!(
            &*b.read().unwrap(),
            &*store
                .exhume_simple_supertype(&b.read().unwrap().id)
                .unwrap()
                .read()
                .unwrap()
        );

        assert_eq!(
            &*a.read().unwrap(),
            &*z.read().unwrap().r1_simple_supertype(&store)[0]
                .read()
                .unwrap()
        );

        let r = Reference::new("this is a reference".to_owned(), &mut store);
        let a = SubtypeA::new("a".to_owned(), &mut store);
        let sa = SuperT::new_subtype_a(&r, &a, &mut store);
        let b = SubtypeB::new(8, &mut store);
        let sb = SuperT::new_subtype_b(&r, &b, &mut store);

        // assert_eq!(&a, store.exhume_subtype_a(&a.id).unwrap());
        assert_eq!(
            &*b.read().unwrap(),
            &*store
                .exhume_subtype_b(&b.read().unwrap().id)
                .unwrap()
                .read()
                .unwrap()
        );
        assert_eq!(
            &*sa.read().unwrap(),
            &*store
                .exhume_super_t(&sa.read().unwrap().id)
                .unwrap()
                .read()
                .unwrap()
        );
        assert_eq!(
            &*sb.read().unwrap(),
            &*store
                .exhume_super_t(&sb.read().unwrap().id)
                .unwrap()
                .read()
                .unwrap()
        );

        assert_eq!(
            &*sa.read().unwrap(),
            &*a.read().unwrap().r2_super_t(&store)[0].read().unwrap()
        );
        assert_eq!(
            &*sb.read().unwrap(),
            &*b.read().unwrap().r2_super_t(&store)[0].read().unwrap()
        );
    }

    #[test]
    fn test_r3() {
        Client::start();

        let mut store = ObjectStore::new();

        let oh_boy = OhBoy::new("The Wall Street Journal".to_owned(), &mut store);
        let z = SimpleSubtypeA::new_oh_boy(&oh_boy, &mut store);
        let henry = Henry::new("Henry".to_owned(), &z, &mut store);

        assert_eq!(
            &*z.read().unwrap(),
            &*henry.read().unwrap().r3_simple_subtype_a(&store)[0]
                .read()
                .unwrap()
        );
        assert_eq!(
            &*henry.read().unwrap(),
            &*z.read().unwrap().r3_henry(&store)[0].read().unwrap()
        );
    }

    #[test]
    fn test_r4() {
        Client::start();

        let mut store = ObjectStore::new();

        let b = SimpleSupertype::new_simple_subtype_b(false, &mut store);
        let baz = Baz::new(1.0, &b, &mut store);

        assert_eq!(
            &*b.read().unwrap(),
            &*baz.read().unwrap().r4_simple_supertype(&store)[0]
                .read()
                .unwrap()
        );
        assert_eq!(
            &*baz.read().unwrap(),
            &*b.read().unwrap().r4_baz(&store)[0].read().unwrap()
        );
    }

    #[test]
    fn test_init() {
        Client::start();

        let mut store = ObjectStore::new();

        let mutable = Borrowed::new_mutable(&mut store);
        let shared = Borrowed::new_shared(&mut store);

        // Just making sure that they exist in the store. This will panic if they
        // do not.
        store
            .exhume_ownership(&mutable.read().unwrap().id())
            .unwrap();
        store
            .exhume_ownership(&shared.read().unwrap().id())
            .unwrap();
    }

    #[test]
    fn test_multi_super_sub() {
        Client::start();

        let mut store = ObjectStore::new();

        #[allow(non_snake_case)]
        let Γ = Gamma::new(3.14, &mut store);
        let α = Alpha::new_gamma("α".to_owned(), &Γ, &mut store);
        let β = Beta::new_gamma("β".to_owned(), &Γ, &mut store);

        assert_eq!(
            &*α.read().unwrap(),
            &*Γ.read().unwrap().r10_alpha(&store)[0].read().unwrap()
        );
        assert_eq!(
            &*β.read().unwrap(),
            &*Γ.read().unwrap().r11_beta(&store)[0].read().unwrap()
        );

        let sf = SuperFoo::new_gamma(&Γ, &mut store);
        let sb = SuperBar::new_gamma(&Γ, &mut store);

        assert_eq!(
            &*sf.read().unwrap(),
            &*Γ.read().unwrap().r13_super_foo(&store)[0].read().unwrap()
        );
        assert_eq!(
            &*sb.read().unwrap(),
            &*Γ.read().unwrap().r12_super_bar(&store)[0].read().unwrap()
        );
    }
}
