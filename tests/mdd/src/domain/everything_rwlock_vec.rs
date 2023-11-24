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
            id: 0,
            name: "rando".to_owned(),
        };

        // Test derive option ✅
        format!("{:?}", r);

        // Test struct creation ✅
        let e = Everything {
            id: 0,
            s_string: "everything".to_owned(),
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

        let r = RandoObject::new("rando".to_owned(), &mut store);
        let e = Everything::new(true, 42.0, 42, "string".to_owned(), &r, &mut store);

        assert_eq!(e.read().unwrap().s_string, "string".to_owned());
        assert_eq!(e.read().unwrap().float, 42.0);
        assert_eq!(e.read().unwrap().bool, true);
        assert_eq!(e.read().unwrap().int, 42);
        assert_eq!(e.read().unwrap().rando, r.read().unwrap().id);

        let r_prime = store.exhume_rando_object(&r.read().unwrap().id).unwrap();
        assert_eq!(&*r.read().unwrap(), &*r_prime.read().unwrap());

        let e_prime = store.exhume_everything(&e.read().unwrap().id).unwrap();
        assert_eq!(&*e.read().unwrap(), &*e_prime.read().unwrap());
    }

    #[test]
    fn test_rel_nav() {
        let mut store = ObjectStore::new();

        let r = RandoObject::new("rando".to_owned(), &mut store);
        let e = Everything::new(true, 42.0, 42, "string".to_owned(), &r, &mut store);

        let r_prime = &e.read().unwrap().r1_rando_object(&store)[0];
        assert_eq!(&*r.read().unwrap(), &*r_prime.read().unwrap());

        let e_prime = &r.read().unwrap().r1_everything(&store)[0];
        assert_eq!(&*e.read().unwrap(), &*e_prime.read().unwrap());
    }
}
