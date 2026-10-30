package soroban

// Version is the version of the Go SDK. It tracks the version of the Rust
// soroban-sdk crate it is ported from.
const Version = "30.0.0-rc.1"

// InterfaceVersion is the ledger protocol version the SDK targets, recorded in
// the contractenvmetav0 section of contracts built with it.
const InterfaceVersion = 30
