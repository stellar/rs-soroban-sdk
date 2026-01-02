use serde::Deserialize;
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use std::{
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
    #[serde_as(as = "LedgerEntryVec")]
    pub ledger_entries: Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>,
}

/// Extended ledger entry that includes the live util ledger sequence. Provides a more compact
/// form of the tuple used in [`LedgerSnapshot::ledger_entries`], to reduce the size of the snapshot
/// when serialized to JSON.
#[derive(Debug, Clone, serde::Deserialize)]
struct LedgerEntryExt {
    entry: Box<LedgerEntry>,
    live_util: Option<u32>,
}

/// Extended ledger entry that includes the live util ledger sequence, and the entry by reference.
/// Used to reduce memory usage during serialization.
#[derive(serde::Serialize)]
struct LedgerEntryExtRef<'a> {
    entry: &'a Box<LedgerEntry>, // Reference = no clone
    live_util: Option<u32>,
}

struct LedgerEntryVec;

impl<'a> SerializeAs<Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>> for LedgerEntryVec {
    fn serialize_as<S>(
        source: &Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq;
        let mut seq = serializer.serialize_seq(Some(source.len()))?;
        for (_, (entry, live_util)) in source {
            seq.serialize_element(&LedgerEntryExtRef {
                entry,
                live_util: *live_util,
            })?;
        }
        seq.end()
    }
}

impl<'de> DeserializeAs<'de, Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>>
    for LedgerEntryVec
{
    fn deserialize_as<D>(
        deserializer: D,
    ) -> Result<Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(untagged)]
        enum Format {
            V2(Vec<LedgerEntryExt>),
            V1(Vec<(Box<LedgerKey>, (Box<LedgerEntry>, Option<u32>))>),
        }

        match Format::deserialize(deserializer)? {
            Format::V2(entries) => Ok(entries
                .into_iter()
                .map(|LedgerEntryExt { entry, live_util }| {
                    let key = Box::new(entry.to_key());
                    (key, (entry, live_util))
                })
                .collect()),
            Format::V1(entries) => Ok(entries),
        }
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

    /// Get the entries in the snapshot.
    pub fn entries(
        &self,
    ) -> impl IntoIterator<Item = (&Box<LedgerKey>, &(Box<LedgerEntry>, Option<u32>))> {
        self.ledger_entries.iter().map(|(k, v)| (k, v))
    }

    /// Replace the entries in the snapshot with the entries in the iterator.
    pub fn set_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, (&'a Box<LedgerEntry>, Option<u32>))>,
    ) {
        self.ledger_entries.clear();
        for (k, e) in entries {
            self.ledger_entries.push((k.clone(), (e.0.clone(), e.1)));
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
            let i = self.ledger_entries.iter().position(|(ik, _)| **ik == **k);
            if let Some((entry, live_until_ledger)) = e {
                let new = (
                    Box::new((**k).clone()),
                    (Box::new((**entry).clone()), *live_until_ledger),
                );
                if let Some(i) = i {
                    self.ledger_entries[i] = new;
                } else {
                    self.ledger_entries.push(new);
                }
            } else if let Some(i) = i {
                self.ledger_entries.swap_remove(i);
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
            protocol_version: 23,
            sequence_number: Default::default(),
            timestamp: Default::default(),
            network_id: Default::default(),
            base_reserve: Default::default(),
            ledger_entries: Vec::default(),
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
        match self.ledger_entries.iter().find(|(k, _)| **k == **key) {
            Some((_, v)) => Ok(Some((Rc::new(*v.0.clone()), v.1))),
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
