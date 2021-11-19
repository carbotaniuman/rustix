#[cfg(target_os = "linux")]
use super::super::c;
#[cfg(target_os = "linux")]
use bitflags::bitflags;

#[cfg(target_os = "linux")]
bitflags! {
    pub struct GetRandomFlags: u32 {
        /// GRND_RANDOM
        const RANDOM = c::GRND_RANDOM;
        /// GRND_NONBLOCK
        const NONBLOCK = c::GRND_NONBLOCK;
    }
}