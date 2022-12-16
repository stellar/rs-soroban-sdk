use std::{
    fs::{create_dir_all, File},
    io::{self, Read, Write},
    path::Path,
    rc::Rc,
};

use soroban_env_host::{
    storage::SnapshotSource,
    xdr::{LedgerEntry, LedgerKey, ScHostStorageErrorCode, ScStatus},
    Host, HostError, LedgerInfo,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io")]
    Io(#[from] io::Error),
    #[error("serde")]
    Serde(#[from] serde_json::Error),
}

/// Ledger snapshot stores a snapshot of a ledger that can be restored for use
/// in environments as a [`LedgerInfo`] and a [`SnapshotSource`].
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LedgerSnapshot {
    pub protocol_version: u32,
    pub sequence_number: u32,
    pub timestamp: u64,
    pub network_passphrase: Vec<u8>,
    pub base_reserve: u32,
    pub ledger_entries: Vec<(Box<LedgerKey>, Box<LedgerEntry>)>,
}

impl LedgerSnapshot {
    // Create a ledger snapshot from ledger info and a set of entries.
    pub fn from<'a>(
        info: LedgerInfo,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, &'a Box<LedgerEntry>)>,
    ) -> Self {
        let mut s = Self::default();
        s.set_ledger_info(info);
        s.set_entries(entries);
        s
    }

    /// Update the snapshot with the state within the given [`Host`].
    ///
    /// The ledger info of the host will overwrite the ledger info in the
    /// snapshot.  The entries in the host's storage will overwrite entries in
    /// the snapshot. Existing entries in the snapshot that are untouched by the
    /// host will remain.
    pub fn update(&mut self, host: &Host) {
        let _result = host.with_ledger_info(|li| {
            self.set_ledger_info(li.clone());
            Ok(())
        });
        let _result = host.with_mut_storage(|s| {
            self.update_entries(&s.map);
            Ok(())
        });
    }

    // Get the ledger info in the snapshot.
    pub fn ledger_info(&self) -> LedgerInfo {
        LedgerInfo {
            protocol_version: self.protocol_version,
            sequence_number: self.sequence_number,
            timestamp: self.timestamp,
            network_passphrase: self.network_passphrase.clone(),
            base_reserve: self.base_reserve,
        }
    }

    /// Set the ledger info in the snapshot.
    pub fn set_ledger_info(&mut self, info: LedgerInfo) {
        self.protocol_version = info.protocol_version;
        self.sequence_number = info.sequence_number;
        self.timestamp = info.timestamp;
        self.network_passphrase = info.network_passphrase;
        self.base_reserve = info.base_reserve;
    }

    /// Get the entries in the snapshot.
    pub fn entries(&self) -> impl IntoIterator<Item = (&Box<LedgerKey>, &Box<LedgerEntry>)> {
        self.ledger_entries.iter().map(|(k, v)| (k, v))
    }

    /// Replace the entries in the snapshot with the entries in the iterator.
    pub fn set_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, &'a Box<LedgerEntry>)>,
    ) {
        self.ledger_entries.clear();
        for (k, e) in entries {
            self.ledger_entries.push((k.clone(), e.clone()));
        }
    }

    /// Update entries in the snapshot by adding or replacing any entries that
    /// have entry in the input iterator, or removing any that does not have an
    /// entry.
    pub fn update_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = &'a (Rc<LedgerKey>, Option<Rc<LedgerEntry>>)>,
    ) {
        for (k, e) in entries {
            let i = self.ledger_entries.iter().position(|(ik, _)| **ik == **k);
            if let Some(e) = e {
                let new = (Box::new((**k).clone()), Box::new((**e).clone()));
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
    // Read in a [`LedgerSnapshot`] from a reader.
    pub fn read(r: impl Read) -> Result<LedgerSnapshot, Error> {
        Ok(serde_json::from_reader::<_, LedgerSnapshot>(r)?)
    }

    // Read in a [`LedgerSnapshot`] from a file.
    pub fn read_file(p: impl AsRef<Path>) -> Result<LedgerSnapshot, Error> {
        Self::read(File::open(p)?)
    }

    // Write a [`LedgerSnapshot`] to a writer.
    pub fn write(&self, w: impl Write) -> Result<(), Error> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    // Write a [`LedgerSnapshot`] to file.
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
            protocol_version: 20,
            sequence_number: Default::default(),
            timestamp: Default::default(),
            network_passphrase: Vec::default(),
            base_reserve: Default::default(),
            ledger_entries: Vec::default(),
        }
    }
}

impl SnapshotSource for &LedgerSnapshot {
    fn get(&self, key: &LedgerKey) -> Result<LedgerEntry, HostError> {
        match self.ledger_entries.iter().find(|(k, _)| k.as_ref() == key) {
            Some((_, v)) => Ok(*v.clone()),
            None => {
                Err(ScStatus::HostStorageError(ScHostStorageErrorCode::AccessToUnknownEntry).into())
            }
        }
    }
    fn has(&self, key: &LedgerKey) -> Result<bool, HostError> {
        Ok(self.ledger_entries.iter().any(|(k, _)| k.as_ref() == key))
    }
}

impl SnapshotSource for LedgerSnapshot {
    fn get(&self, key: &LedgerKey) -> Result<LedgerEntry, HostError> {
        <_ as SnapshotSource>::get(&self, key)
    }
    fn has(&self, key: &LedgerKey) -> Result<bool, HostError> {
        <_ as SnapshotSource>::has(&self, key)
    }
}
