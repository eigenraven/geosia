use std::fmt::{Debug, Formatter};

use bytemuck::{Pod, TransparentWrapper, Zeroable};
use serde::{Deserialize, Serialize};

/**
 * A Block identifier used to uniquely identify a registered block variant.
 * Some bits are dedicated for faster property lookup to avoid an extra registry indirection, they must be validated against the registry on deserialization.
 *
 * `[ registry id (32 bits) | for future use | render_mode (2b) | solid_side (6b) | shape (6b) ]`
 */
#[derive(
    Copy,
    Clone,
    Default,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    Zeroable,
    Pod,
    TransparentWrapper,
)]
#[repr(transparent)]
pub struct BlockId(u64);

impl BlockId {
    pub fn from_bits(registry_id: u32, shape_id: u8, solid_sides: u8, render_mode: u8) -> Self {
        Self(
            (registry_id as u64) << 3
                | (shape_id & 0b111111) as u64
                | ((solid_sides & 0b111111) as u64) << 6
                | ((render_mode & 0b11) as u64) << 12,
        )
    }

    pub fn registry_id_bits(self) -> u32 {
        ((self.0 >> 32) & 0xFFFF_FFFF) as u32
    }

    pub fn shape_id_bits(self) -> u8 {
        (self.0 & 0b111111) as u8
    }

    pub fn solid_sides_bits(self) -> u8 {
        ((self.0 >> 6) & 0b111111) as u8
    }

    pub fn render_mode_bits(self) -> u8 {
        ((self.0 >> 12) & 0b11) as u8
    }
}

impl Debug for BlockId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BlockId(reg=0x{:08X}, shape={}, solid_sides={}, render_mode={})",
            self.registry_id_bits(),
            self.shape_id_bits(),
            self.solid_sides_bits(),
            self.render_mode_bits()
        )
    }
}
