//! A type for not generating anything
//!
use std::sync::RwLock;

use fnv::FnvHashMap as HashMap;
use sarzak::{
    lu_dog::store::ObjectStore as LuDogStore, mc::Result, v2::domain::Domain,
    woog::store::ObjectStore as WoogStore,
};
use uuid::Uuid;

use crate::{
    codegen::{
        buffer::Buffer,
        generator::{FileGenerator, GenerationAction},
    },
    options::GraceConfig,
};

pub(crate) struct NullGenerator;

impl NullGenerator {
    pub(crate) fn new() -> Box<dyn FileGenerator> {
        Box::new(Self)
    }
}

impl FileGenerator for NullGenerator {
    fn generate(
        &self,
        _config: &GraceConfig,
        _domain: &Domain,
        _woog: &Option<&mut WoogStore>,
        _lu_dog: &Option<&RwLock<LuDogStore>>,
        _imports: &Option<&HashMap<String, Domain>>,
        _package: &str,
        _module: &str,
        _obj_id: Option<&Uuid>,
        _buffer: &mut Buffer,
    ) -> Result<GenerationAction> {
        Ok(GenerationAction::Skip)
    }
}
