use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{Read, Write},
    path::Path,
};

use soroban_env_host::{
    storage::SnapshotSource,
    xdr::{LedgerEntry, LedgerKey, ScHostStorageErrorCode, ScStatus},
    HostError, LedgerInfo,
};

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
    pub entries: HashMap<Box<LedgerKey>, Box<LedgerEntry>>,
}

impl LedgerSnapshot {
    pub fn from<'a>(
        info: LedgerInfo,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, &'a Box<LedgerEntry>)>,
    ) -> Self {
        let mut s = Self::default();
        s.set_ledger_info(info);
        s.set_entries(entries);
        s
    }

    pub fn to_ledger_info(&self) -> LedgerInfo {
        LedgerInfo {
            protocol_version: self.protocol_version,
            sequence_number: self.sequence_number,
            timestamp: self.timestamp,
            network_passphrase: self.network_passphrase.clone(),
            base_reserve: self.base_reserve,
        }
    }

    pub fn set_ledger_info(&mut self, info: LedgerInfo) {
        self.protocol_version = info.protocol_version;
        self.sequence_number = info.sequence_number;
        self.timestamp = info.timestamp;
        self.network_passphrase = info.network_passphrase;
        self.base_reserve = info.base_reserve;
    }

    pub fn entries<'a>(
        &'a self,
    ) -> impl IntoIterator<Item = (&'a Box<LedgerKey>, &'a Box<LedgerEntry>)> {
        self.entries.iter()
    }

    pub fn set_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, &'a Box<LedgerEntry>)>,
    ) {
        for (k, e) in entries {
            self.entries.insert(k.clone(), e.clone());
        }
    }

    pub fn update_entries<'a>(
        &mut self,
        entries: impl IntoIterator<Item = (&'a Box<LedgerKey>, &'a Option<Box<LedgerEntry>>)>,
    ) {
        for (k, e) in entries {
            if let Some(e) = e {
                self.entries.insert(k.clone(), e.clone());
            } else {
                self.entries.remove(k);
            }
        }
    }
}

impl LedgerSnapshot {
    pub fn read(r: impl Read) -> Result<LedgerSnapshot, Box<dyn Error>> {
        Ok(serde_json::from_reader::<_, LedgerSnapshot>(r)?)
    }

    pub fn read_file(p: impl AsRef<Path>) -> Result<LedgerSnapshot, Box<dyn Error>> {
        Self::read(File::open(p)?)
    }

    pub fn write(&self, w: impl Write) -> Result<(), Box<dyn Error>> {
        Ok(serde_json::to_writer_pretty(w, self)?)
    }

    pub fn write_file(&self, p: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        self.write(File::open(p)?)
    }
}

impl Default for LedgerSnapshot {
    fn default() -> Self {
        Self {
            entries: HashMap::default(),
            protocol_version: 20,
            sequence_number: Default::default(),
            timestamp: Default::default(),
            network_passphrase: Vec::default(),
            base_reserve: Default::default(),
        }
    }
}

impl SnapshotSource for &LedgerSnapshot {
    fn get(&self, key: &LedgerKey) -> Result<LedgerEntry, HostError> {
        match self.entries.get(key) {
            Some(v) => Ok(*v.clone()),
            None => {
                Err(ScStatus::HostStorageError(ScHostStorageErrorCode::AccessToUnknownEntry).into())
            }
        }
    }
    fn has(&self, key: &LedgerKey) -> Result<bool, HostError> {
        Ok(self.entries.contains_key(key))
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
