// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"everything-struct-definition-file"}}}
use crate::app::everything::UUID_NS;
use uuid::Uuid;

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-referrer-use-statements"}}}
use crate::app::everything::types::rando_object::RandoObject;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-documentation"}}}
/// An object, with everything on it!
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-definition"}}}
#[derive(Debug, PartialEq)]
pub struct Everything<'a> {
    pub bool: bool,
    pub float: f64,
    pub id: Uuid,
    pub int: i64,
    pub s_string: String,
    /// R1: [`Everything`] 'points at' [`RandoObject`]
    pub rando: &'a RandoObject,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"everything-struct-implementation"}}}
// {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-new"}}}
// {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
impl<'a> Everything<'a> {
    // {"magic":"","directive":{"Start":{"directive":"comment-orig","tag":"everything-struct-impl-new"}}}
    /// Inter a new Everything in the store, and return it's `id`.
    //     pub fn new(
    //         bool: bool,
    //         float: f64,
    //         int: i64,
    //         string: String,
    //         rando: &RandoObject,
    //     ) -> Everything {
    //     pub fn new(bool: bool, float: f64, int: f64, string: f64, rando: &RandoObject) -> Everything {
    pub fn new(
        bool: bool,
        float: f64,
        int: i64,
        //         string: String,
        s_string: String,
        rando: &RandoObject,
    ) -> Everything {
        //         let id = Uuid::new_v5(
        //             &UUID_NS,
        //             format!("{}:{}:{}:{}:{:?}", bool, float, int, string, rando).as_bytes(),
        //         );
        let id = Uuid::new_v4();
        Everything {
            bool: bool,
            float: float,
            int: int,
            //             string: string,
            s_string: s_string,
            rando: rando,
            id,
        }
    }
    // {"magic":"","directive":{"End":{"directive":"comment-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}
