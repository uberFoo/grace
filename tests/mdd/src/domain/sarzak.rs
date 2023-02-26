//! And so we have arrived, finally -- sarzak!
//!
use ::uuid::{uuid, Uuid};

pub mod store;
pub mod types;

pub use store::ObjectStore;
pub use types::*;

// not sure really
pub const UUID_NS: Uuid = uuid!("88cd70e0-c065-5c4b-b1ce-69194ab4d0cb");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_persist() {
        let mut store = ObjectStore::new();

        let t = Ty::new_string();
        let o = Object::new(
            "A Widget".to_owned(),
            "W".to_owned(),
            "Widget".to_owned(),
            &mut store,
        );
        let a = Attribute::new("froggles".to_owned(), Some(&o), &t, &mut store);

        store.persist("tmp/models").unwrap();

        let bodega = ObjectStore::load("tmp/models").unwrap();

        assert_eq!(&t, bodega.exhume_ty(&t.id()).unwrap());
        assert_eq!(&o, bodega.exhume_object(&o.id).unwrap());
        assert_eq!(&a, bodega.exhume_attribute(&a.id).unwrap());
    }
}
