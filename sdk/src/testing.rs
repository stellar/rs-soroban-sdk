#![allow(unused_variables)]

pub mod fs;
pub(crate) mod host;
pub mod mem;

pub use host::swap_mock_host;
pub use host::MockHost;
