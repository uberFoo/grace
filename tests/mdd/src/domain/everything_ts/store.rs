//! domain::everything_ts Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_ts-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Everything`]
//! * [`RandoObject`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_ts-object-store-definition"}}}
use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    time::SystemTime,
};

use fnv::FnvHashMap as HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::everything_ts::types::{Everything, RandoObject};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    everything: HashMap<Uuid, (Everything, SystemTime)>,
    rando_object: HashMap<Uuid, (RandoObject, SystemTime)>,
    rando_object_by_name: HashMap<String, (RandoObject, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let store = Self {
            everything: HashMap::default(),
            rando_object: HashMap::default(),
            rando_object_by_name: HashMap::default(),
        };

        // Initialize Singleton Subtypes

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_ts-object-store-methods"}}}
    /// Inter [`Everything`] into the store.
    ///
    pub fn inter_everything(&mut self, everything: Everything) {
        self.everything
            .insert(everything.id, (everything, SystemTime::now()));
    }

    /// Exhume [`Everything`] from the store.
    ///
    pub fn exhume_everything(&self, id: &Uuid) -> Option<&Everything> {
        self.everything.get(id).map(|everything| &everything.0)
    }

    /// Exhume [`Everything`] from the store — mutably.
    ///
    pub fn exhume_everything_mut(&mut self, id: &Uuid) -> Option<&mut Everything> {
        self.everything
            .get_mut(id)
            .map(|everything| &mut everything.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Everything>`.
    ///
    pub fn iter_everything(&self) -> impl Iterator<Item = &Everything> {
        self.everything.values().map(|everything| &everything.0)
    }

    /// Get the timestamp for Everything.
    ///
    pub fn everything_timestamp(&self, everything: &Everything) -> SystemTime {
        self.everything
            .get(&everything.id)
            .map(|everything| everything.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`RandoObject`] into the store.
    ///
    pub fn inter_rando_object(&mut self, rando_object: RandoObject) {
        let value = (rando_object, SystemTime::now());
        self.rando_object.insert(value.0.id, value.clone());
        self.rando_object_by_name
            .insert(value.0.name.clone(), value);
    }

    /// Exhume [`RandoObject`] from the store.
    ///
    pub fn exhume_rando_object(&self, id: &Uuid) -> Option<&RandoObject> {
        self.rando_object
            .get(id)
            .map(|rando_object| &rando_object.0)
    }

    /// Exhume [`RandoObject`] from the store — mutably.
    ///
    pub fn exhume_rando_object_mut(&mut self, id: &Uuid) -> Option<&mut RandoObject> {
        self.rando_object
            .get_mut(id)
            .map(|rando_object| &mut rando_object.0)
    }

    /// Exhume [`RandoObject`] from the store by name.
    ///
    pub fn exhume_rando_object_by_name(&self, name: &str) -> Option<&RandoObject> {
        self.rando_object_by_name
            .get(name)
            .map(|rando_object| &rando_object.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RandoObject>`.
    ///
    pub fn iter_rando_object(&self) -> impl Iterator<Item = &RandoObject> {
        self.rando_object
            .values()
            .map(|rando_object| &rando_object.0)
    }

    /// Get the timestamp for RandoObject.
    ///
    pub fn rando_object_timestamp(&self, rando_object: &RandoObject) -> SystemTime {
        self.rando_object
            .get(&rando_object.id)
            .map(|rando_object| rando_object.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"domain::everything_ts-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        fs::create_dir_all(&path)?;

        let bin_path = path.clone().join("everything.bin");
        let mut bin_file = fs::File::create(bin_path)?;
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        bin_file.write_all(&encoded)?;

        let path = path.join("everything.json");
        fs::create_dir_all(&path)?;

        // Persist Everything.
        {
            let path = path.join("everything");
            fs::create_dir_all(&path)?;
            for everything_tuple in self.everything.values() {
                let path = path.join(format!("{}.json", everything_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Everything, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != everything_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &everything_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &everything_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.everything.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Rando Object.
        {
            let path = path.join("rando_object");
            fs::create_dir_all(&path)?;
            for rando_object_tuple in self.rando_object.values() {
                let path = path.join(format!("{}.json", rando_object_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (RandoObject, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != rando_object_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &rando_object_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &rando_object_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.rando_object.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("everything.json");

        let mut store = Self::new();

        // Load Everything.
        {
            let path = path.join("everything");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let everything: (Everything, SystemTime) = serde_json::from_reader(reader)?;
                store.everything.insert(everything.0.id, everything);
            }
        }

        // Load Rando Object.
        {
            let path = path.join("rando_object");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let rando_object: (RandoObject, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .rando_object_by_name
                    .insert(rando_object.0.name.clone(), rando_object.clone());
                store.rando_object.insert(rando_object.0.id, rando_object);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
