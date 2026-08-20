// The `sparse` argument on `contractevent` only applies to the map data format,
// and must be rejected on the other data formats.
use soroban_sdk::contractevent;

#[contractevent(data_format = "vec", sparse = true)]
pub struct Ev {
    pub a: u32,
}

#[contractevent(data_format = "single-value", sparse = true)]
pub struct Ev2 {
    pub a: u32,
}

fn main() {}
