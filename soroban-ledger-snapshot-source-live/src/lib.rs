use soroban_ledger_fetch_from_rpc::{get_ledger_entry, parse_ledger_entry};
use soroban_sdk::{
    testutils::{HostError, SnapshotSource, SnapshotSourceInput},
    xdr::{LedgerEntry, LedgerKey, ScErrorCode, ScErrorType},
};
use std::rc::Rc;

/// Snapshot source that fetches ledger entries from stellar-rpc
pub struct RpcSnapshotSource {
    rpc_url: String,
}

impl RpcSnapshotSource {
    /// Create a new RPC snapshot source
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
        }
    }
}

impl SnapshotSource for RpcSnapshotSource {
    fn get(
        &self,
        key: &Rc<LedgerKey>,
    ) -> Result<Option<(Rc<LedgerEntry>, Option<u32>)>, HostError> {
        let mut buffer = Vec::new();
        let result = get_ledger_entry(&self.rpc_url, key.as_ref(), &mut buffer);

        if let Err(err) = result {
            eprintln!("Error fetching from RPC: {err:?}");
            return Err(HostError::from(soroban_sdk::Error::from((
                ScErrorType::Storage,
                ScErrorCode::InternalError,
            ))));
        }

        let parse_result = parse_ledger_entry(buffer.as_slice());
        match parse_result {
            Ok(Some((entry, ttl))) => Ok(Some((Rc::new(entry), ttl))),
            Ok(None) => Ok(None),
            Err(err) => {
                eprintln!("Error parsing RPC response: {err:?}");
                Err(HostError::from(soroban_sdk::Error::from((
                    ScErrorType::Storage,
                    ScErrorCode::InternalError,
                ))))
            }
        }
    }
}

impl From<RpcSnapshotSource> for SnapshotSourceInput {
    fn from(source: RpcSnapshotSource) -> Self {
        Self {
            source: Rc::new(source),
            ledger_info: None,
            snapshot: None,
        }
    }
}
