//! Everything Domain
//!
//! This file was generated by: `sarzak new "everything"`.
use uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// everything
pub const UUID_NS: Uuid = uuid!("68f2a75c-4397-5ee0-9bae-86b95bd1b866");

#[cfg(test)]
mod tests {
    use super::*;

    use everything::Everything;
    use rando_object::RandoObject;
    // use store::ObjectStore;

    #[test]
    fn test_structs() {
        // Test struct creation ✅
        let r = RandoObject {
            id: Uuid::new_v5(&UUID_NS, b"rando"),
        };

        // Test derive option ✅
        format!("{:?}", r);

        // Test struct creation ✅
        let e = Everything {
            id: Uuid::new_v5(&UUID_NS, b"everything"),
            string: "everything".to_owned(),
            float: 42.0,
            bool: true,
            int: 42,
            rando: r.id,
        };

        // Test derive option ✅
        format!("{:?}", e);
    }

    #[test]
    fn test_new() {
        let mut store = ObjectStore::new();

        let r = RandoObject::new(&mut store);
        let e = Everything::new(true, 42.0, 42, "string".to_owned(), &r, &mut store);

        assert_eq!(e.string, "string".to_owned());
        assert_eq!(e.float, 42.0);
        assert_eq!(e.bool, true);
        assert_eq!(e.int, 42);
        assert_eq!(e.rando, r.id);

        let r_prime = store.exhume_rando_object(&r.id).unwrap();
        assert_eq!(&r, r_prime);

        let e_prime = store.exhume_everything(&e.id).unwrap();
        assert_eq!(&e, e_prime);
    }

    #[test]
    fn test_rel_nav() {
        let mut store = ObjectStore::new();

        let r = RandoObject::new(&mut store);
        let e = Everything::new(true, 42.0, 42, "string".to_owned(), &r, &mut store);

        let r_prime = e.rando_object(&store);
        assert_eq!(vec![&r], r_prime);

        let e_prime = r.everything(&store);
        assert_eq!(vec![&e], e_prime);
    }
}
