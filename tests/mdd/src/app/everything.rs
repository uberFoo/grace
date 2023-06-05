//! Everything Domain
//!
//! This file was generated by: `sarzak new "everything"`.
use uuid::{uuid, Uuid};

pub mod types;

pub use types::*;

// everything
pub const UUID_NS: Uuid = uuid!("68f2a75c-4397-5ee0-9bae-86b95bd1b866");

#[cfg(test)]
mod tests {
    use super::*;

    use everything::Everything;
    use rando_object::RandoObject;

    #[test]
    fn test_structs() {
        // Test struct creation ✅
        let r = RandoObject {
            id: Uuid::new_v5(&UUID_NS, b"rando"),
            name: "rando".to_owned(),
        };

        // Test derive option ✅
        format!("{:?}", r);

        // Test struct creation ✅
        let e = Everything {
            id: Uuid::new_v5(&UUID_NS, b"everything"),
            s_string: "everything".to_owned(),
            float: 42.0,
            bool: true,
            int: 42,
            rando: &r,
        };

        // Test derive option ✅
        format!("{:?}", e);
    }

    #[test]
    fn test_new() {
        let r = RandoObject::new("rando".to_owned());
        let e = Everything::new(true, 42.0, 42, "string".to_owned(), &r);
        assert_eq!(e.s_string, "string".to_owned());
        assert_eq!(e.float, 42.0);
        assert_eq!(e.bool, true);
        assert_eq!(e.int, 42);
        assert_eq!(e.rando, &r);
    }
}
