#[cfg(target_pointer_width = "32")]
pub fn usize_to_u32(u: usize) -> Option<u32> {
    Some(u as u32)
}

#[cfg(target_pointer_width = "64")]
pub fn usize_to_u32(u: usize) -> Option<u32> {
    u.try_into().ok()
}

#[cfg(target_pointer_width = "32")]
pub fn u32_to_usize(u: u32) -> usize {
    u as usize
}

#[cfg(target_pointer_width = "64")]
pub fn u32_to_usize(u: u32) -> usize {
    u as usize
}
