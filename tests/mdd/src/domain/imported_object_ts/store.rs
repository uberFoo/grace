//! domain::imported_object_ts Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object_ts-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`AnotherObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object_ts-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use rustc_hash::FxHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::imported_object_ts::types::AnotherObject;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    another_object: HashMap<Uuid, (AnotherObject, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            another_object: HashMap::default(),
        };

        // Initialize Singleton Subtypes
        // 💥 Look at how beautiful this generated code is for super/sub-type graphs!
        // I remember having a bit of a struggle making it work. It's recursive, with
        // a lot of special cases, and I think it calls other recursive functions...💥

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object_ts-object-store-methods"}}}
    /// Inter (insert) [`AnotherObject`] into the store.
    ///
    pub fn inter_another_object(&mut self, another_object: AnotherObject) {
        self.another_object
            .insert(another_object.id, (another_object, SystemTime::now()));
    }

    /// Exhume (get) [`AnotherObject`] from the store.
    ///
    pub fn exhume_another_object(&self, id: &Uuid) -> Option<&AnotherObject> {
        self.another_object
            .get(id)
            .map(|another_object| &another_object.0)
    }

    /// Exorcise (remove) [`AnotherObject`] from the store.
    ///
    pub fn exorcise_another_object(&mut self, id: &Uuid) -> Option<AnotherObject> {
        self.another_object
            .remove(id)
            .map(|another_object| another_object.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AnotherObject>`.
    ///
    pub fn iter_another_object(&self) -> impl Iterator<Item = &AnotherObject> {
        self.another_object
            .values()
            .map(|another_object| &another_object.0)
    }

    /// Get the timestamp for AnotherObject.
    ///
    pub fn another_object_timestamp(&self, another_object: &AnotherObject) -> SystemTime {
        self.another_object
            .get(&another_object.id)
            .map(|another_object| another_object.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::imported_object_ts-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a a bincode file.
    pub fn persist_bincode<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let mut bin_file = fs::File::create(path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;
        Ok(())
    }

    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(path)?;

        let path = path.join("imported_object.json");
        fs::create_dir_all(&path)?;

        // Persist Another Object.
        {
            let path = path.join("another_object");
            fs::create_dir_all(&path)?;
            for another_object_tuple in self.another_object.values() {
                let path = path.join(format!("{}.json", another_object_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (AnotherObject, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != another_object_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &another_object_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &another_object_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split('.').next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.another_object.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    pub fn from_bincode(code: &[u8]) -> io::Result<Self> {
        Ok(bincode::deserialize(code).unwrap())
    }

    /// The store is as a bincode file.
    pub fn load_bincode<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let bin_file = fs::File::open(path)?;
        Ok(bincode::deserialize_from(bin_file).unwrap())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("imported_object.json");

        let mut store = Self::new();

        // Load Another Object.
        {
            let path = path.join("another_object");
            let entries = fs::read_dir(path)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let another_object: (AnotherObject, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .another_object
                    .insert(another_object.0.id, another_object);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
