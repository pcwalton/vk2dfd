// vk2dfd/src/lib.rs

#![doc = include_str!("../README.md")]

use crate::table::VK_TO_DFD;
use thiserror::Error;

mod table;

#[cfg(test)]
mod test;

/// A Vulkan format, one of the `VK_FORMAT_` constants.
/// 
/// This is just a type alias for `u32` for maximum compatibility with the Rust Vulkan ecosystem.
pub type VkFormat = u32;

/// Errors that can occur when translating a Vulkan format into a DFD.
#[derive(Error, Debug)]
pub enum Error {
    /// The Vulkan type is unknown.
    #[error("No DFD is available for that Vulkan type")]
    UnknownType,
}

/// Converts a Vulkan format to the appropriate descriptor.
/// 
/// Returns the descriptor as a static slice of `u32`s, or an [enum@Error] on
/// failure. If written to a KTX or KTX2 file, the descriptor must be written in
/// little endian byte order per the specification. The slice includes the
/// leading size word.
#[inline]
pub fn vk2dfd(vulkan_format: VkFormat) -> Result<&'static [u32], Error> {
    VK_TO_DFD
        .get(&vulkan_format)
        .cloned()
        .ok_or(Error::UnknownType)
}
