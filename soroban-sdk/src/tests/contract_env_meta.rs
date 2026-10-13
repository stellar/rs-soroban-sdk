// Contracts built with the SDK embed the protocol version of the env in the
// contractenvmetav0 custom section.
use stellar_xdr::{Limited, Limits, ReadXdr, ScEnvMetaEntry};

const WASM: &[u8] = include_bytes!("../../../target/wasm32v1-none/release/test_add_u64.wasm");

#[test]
fn test_env_meta_protocol_version() {
    let section = wasmparser::Parser::new(0)
        .parse_all(WASM)
        .find_map(|p| match p.unwrap() {
            wasmparser::Payload::CustomSection(s) if s.name() == "contractenvmetav0" => {
                Some(s.data())
            }
            _ => None,
        })
        .unwrap();
    let entries = ScEnvMetaEntry::read_xdr_iter(&mut Limited::new(section, Limits::none()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        entries,
        [ScEnvMetaEntry::ScEnvMetaKindInterfaceVersion(
            soroban_env_host::VERSION.interface
        )]
    );
}
