use serde_with::serde_as;
use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{create_dir_all, File},
    io::{self, BufReader, Read, Write},
    path::Path,
    rc::Rc,
};

use soroban_env_host::{
    storage::SnapshotSource,
    xdr::{LedgerEntry, LedgerKey},
    HostError, LedgerInfo,
};

#[cfg(test)]
mod tests;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io")]
    Io(#[from] io::Error),
    #[error("serde")]
    Serde(#[from] serde_json::Error),
}

/// Ledger snapshot stores a snapshot of a ledger that can be restored for use
/// in environments as a [`LedgerInfo`] and a [`SnapshotSource`].
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LedgerSnapshot {
    pub protocol_version: u32,
    pub sequence_number: u32,
    pub timestamp: u64,
    #[serde_as(as = "serde_with::hex::Hex")]
    pub network_id: [u8; 32],
    pub base_reserve: u32,
    pub min_persistent_entry_ttl: u32,
    pub min_temp_entry_ttl: u32,
    pub max_entry_ttl: u32,
    ledger_entries: LedgerEntries,
}

/// Extended ledger entry that includes the live util ledger sequence. Provides a more compact
/// form of the entry storage, to reduce the size of the snapshot when serialized to JSON.
#[derive(Debug, Clone, serde::Deserialize)]
struct LedgerEntryExt {
    entry: Box<LedgerEntry>,
    live_until: Option<u32>,
}

/// Extended ledger entry that includes the live util ledger sequence, and the entry by reference.
/// Used to reduce memory usage during serialization.
#[derive(serde::Serialize)]
struct LedgerEntryExtRef<'a> {
    entry: &'a Box<LedgerEntry>, // Reference = no clone
    live_until: Option<u32>,
}

/// Storage for ledger entries. Uses a [`HashMap`] for O(1) keyed
/// read/write access and a [`Vec`] of keys to preserve insertion order for serialization.
/// Removed keys are left as tombstones in `keys` and filtered out during iteration.
#[derive(Clone, Debug, Default, Eq)]
pub struct LedgerEntries {
    map: HashMap<Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>)>,
    keys: Vec<Box<LedgerKey>>,
}

impl PartialEq for LedgerEntries {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
            && self
                .keys
                .iter()
                .filter(|k| self.map.contains_key(k.as_ref()))
                .eq(other
                    .keys
                    .iter()
                    .filter(|k| other.map.contains_key(k.as_ref())))
    }
}

impl LedgerEntries {
    /// Get the entry for a given key, if it exists.
    fn get(&self, key: &LedgerKey) -> Option<&(Box<LedgerEntry>, Option<u32>)> {
        self.map.get(key)
    }

    /// Insert or replace the entry for a given key.
    fn insert(&mut self, key: Box<LedgerKey>, value: (Box<LedgerEntry>, Option<u32>)) {
        match self.map.entry(key) {
            Entry::Occupied(mut e) => {
                e.insert(value);
            }
            Entry::Vacant(e) => {
                self.keys.push(e.key().clone());
                e.insert(value);
            }
        }
    }

    /// Remove the entry for a given key, if it exists.
    fn remove(&mut self, key: &LedgerKey) {
        self.map.remove(key);
    }

    /// Iterate over the entries in insertion order
    fn iter(&self) -> impl Iterator<Item = (&Box<LedgerKey>, &(Box<LedgerEntry>, Option<u32>))> {
        self.keys
            .iter()
            .filter_map(|k| self.map.get(k).map(|v| (k, v)))
    }

    /// Clear all entries from the storage.
    fn clear(&mut self) {
        self.keys.clear();
        self.map.clear();
    }
}

impl serde::Serialize for LedgerEntries {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(self.map.len()))?;
        for (_, (entry, live_until)) in self.iter() {
            seq.serialize_element(&LedgerEntryExtRef {
                entry,
                live_until: *live_until,
            })?;
        }
        seq.end()
    }
}

impl<'de> serde::Deserialize<'de> for LedgerEntries {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Format {
            V2(Vec<LedgerEntryExt>),
            V1(Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>),
        }
        let mut entries = LedgerEntries::default();
        match Format::deserialize(deserializer)? {
            Format::V2(raw) => {
                for LedgerEntryExt { entry, live_until } in raw {
                    let key = Box::new(entry.to_key());
                    entries.insert(key, (entry, live_until));
                }
            }
            Format::V1(raw) => {
                for (key, value) in raw {
                    entries.insert(key, value);
                }
            }
        }
        Ok(entries)
    }
}

impl LedgerSnapshot {
    /// Create a [`LedgerSnapshot`] from [`LedgerInfo`] and a set of entries.
    pub fn from<'a>(
        info: LedgerInfo,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, (&'a Box<LedgerEntry>, Option<u32>))>,
    ) -> Self {
        let mut s = Self::default();
        s.set_ledger_info(info);
        s.set_entries(entries);
        s
    }

    /// Update the snapshot with the state within the given [`soroban_env_host::Host`].
    ///
    /// The ledger info of the host will overwrite the ledger info in the
    /// snapshot.  The entries in the host's storage will overwrite entries in
    /// the snapshot. Existing entries in the snapshot that are untouched by the
    /// host will remain.
    #[cfg(feature = "testutils")]
    pub fn update(&mut self, host: &soroban_env_host::Host) {
        let _result = host.with_ledger_info(|li| {
            self.set_ledger_info(li.clone());
            Ok(())
        });
        self.update_entries(&host.get_stored_entries().unwrap());
    }

    /// Get the ledger info in the snapshot.
    pub fn ledger_info(&self) -> LedgerInfo {
        LedgerInfo {
            protocol_version: self.protocol_version,
            sequence_number: self.sequence_number,
            timestamp: self.timestamp,
            network_id: self.network_id,
            base_reserve: self.base_reserve,
            min_persistent_entry_ttl: self.min_persistent_entry_ttl,
            min_temp_entry_ttl: self.min_temp_entry_ttl,
            max_entry_ttl: self.max_entry_ttl,
        }
    }

    /// Set the ledger info in the snapshot.
    pub fn set_ledger_info(&mut self, info: LedgerInfo) {
        self.protocol_version = info.protocol_version;
        self.sequence_number = info.sequence_number;
        self.timestamp = info.timestamp;
        self.network_id = info.network_id;
        self.base_reserve = info.base_reserve;
        self.min_persistent_entry_ttl = info.min_persistent_entry_ttl;
        self.min_temp_entry_ttl = info.min_temp_entry_ttl;
        self.max_entry_ttl = info.max_entry_ttl;
    }

    /// Count the number of entries in the snapshot.
    pub fn count_entries(&self) -> usize {
        self.ledger_entries.map.len()
    }

    /// Iterate over all the entries in the snapshot, in insertion order.
    pub fn entries(
        &self,
    ) -> impl IntoIterator<Item = (&Box<LedgerKey>, &(Box<LedgerEntry>, Option<u32>))> {
        self.ledger_entries.iter()
    }

    /// Replace the entries in the snapshot with the entries in the iterator.
    pub fn set_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, (&'a Box<LedgerEntry>, Option<u32>))>,
    ) {
        self.ledger_entries.clear();
        for (k, e) in entries {
            self.ledger_entries.insert(k.clone(), (e.0.clone(), e.1));
        }
    }

    /// Update entries in the snapshot by adding or replacing any entries that
    /// have entry in the input iterator, or removing any that does not have an
    /// entry.
    pub fn update_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = &'a (Rc<LedgerKey>, Option<(Rc<LedgerEntry>, Option<u32>)>)>,
    ) {
        for (k, e) in entries {
            if let Some((entry, live_until_ledger)) = e {
                self.ledger_entries.insert(
                    Box::new((**k).clone()),
                    (Box::new((**entry).clone()), *live_until_ledger),
                );
            } else {
                self.ledger_entries.remove(k);
            }
        }
    }
}

impl LedgerSnapshot {
    /// Read in a [`LedgerSnapshot`] from a reader.
    pub fn read(r: impl Read) -> Result<LedgerSnapshot, Error> {
        Ok(serde_json::from_reader::<_, LedgerSnapshot>(r)?)
    }

    /// Read in a [`LedgerSnapshot`] from a file.
    pub fn read_file(p: impl AsRef<Path>) -> Result<LedgerSnapshot, Error> {
        let reader = BufReader::new(File::open(p)?);
        Self::read(reader)
    }

    /// Write a [`LedgerSnapshot`] to a writer.
    pub fn write(&self, w: impl Write) -> Result<(), Error> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    /// Write a [`LedgerSnapshot`] to file.
    pub fn write_file(&self, p: impl AsRef<Path>) -> Result<(), Error> {
        let p = p.as_ref();
        if let Some(dir) = p.parent() {
            if !dir.exists() {
                create_dir_all(dir)?;
            }
        }
        self.write(File::create(p)?)
    }
}

impl Default for LedgerSnapshot {
    fn default() -> Self {
        Self {
            protocol_version: 25,
            sequence_number: Default::default(),
            timestamp: Default::default(),
            network_id: Default::default(),
            base_reserve: Default::default(),
            ledger_entries: LedgerEntries::default(),
            min_persistent_entry_ttl: Default::default(),
            min_temp_entry_ttl: Default::default(),
            max_entry_ttl: Default::default(),
        }
    }
}

impl SnapshotSource for &LedgerSnapshot {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
        match self.ledger_entries.get(key) {
            Some(v) => Ok(Some((Rc::new(*v.0.clone()), v.1))),
            None => Ok(None),
        }
    }
}

impl SnapshotSource for LedgerSnapshot {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
        <_ as SnapshotSource>::get(&self, key)
    }
}
