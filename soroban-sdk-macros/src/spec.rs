use crate::DEFAULT_XDR_RW_LIMITS;
use flate2::write::GzEncoder;

use stellar_xdr::curr as stellar_xdr;
use stellar_xdr::{Limited, ScSpecEntry, WriteXdr as _};

pub const SECTION_NAME: &str = "contractspecv0gzip";

pub fn to_xdr_gzip(entry: &ScSpecEntry) -> Vec<u8> {
    let mut compressor = GzEncoder::new(Vec::new(), flate2::Compression::best());
    entry
        .write_xdr(&mut Limited::new(&mut compressor, DEFAULT_XDR_RW_LIMITS))
        .unwrap();
    compressor.finish().unwrap()
}
