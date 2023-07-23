//! A collection of strongly typed newtype wrappers for the various coordinate formats within the game's world and related constants.

use std::ops::{Add, Deref};

use bevy_math::IVec3;
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Length of a side of a block in meters
pub const BLOCK_DIM: f32 = 0.5;

/// Length of a side of a chunk in blocks
pub const CHUNK_DIM: i32 = 32;
/// Length of a side of a chunk in blocks
pub const CHUNK_DIMZ: usize = CHUNK_DIM as usize;
/// Number of blocks on the face of a chunk
pub const CHUNK_DIM2: i32 = CHUNK_DIM * CHUNK_DIM;
/// Number of blocks on the face of a chunk
pub const CHUNK_DIM2Z: usize = (CHUNK_DIM * CHUNK_DIM) as usize;
/// Number of blocks in the volume of the chunk
pub const CHUNK_DIM3: i32 = CHUNK_DIM * CHUNK_DIM * CHUNK_DIM;
/// Number of blocks in the volume of the chunk
pub const CHUNK_DIM3Z: usize = (CHUNK_DIM * CHUNK_DIM * CHUNK_DIM) as usize;
/// Chunk dimensions in blocks as a [IVec3] for convenience
pub const CHUNK_DIM3V: IVec3 = IVec3::splat(CHUNK_DIM);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
#[error("Given coordinates were outside of chunk boundaries: {0}")]
pub struct InChunkVecError(IVec3);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Error)]
#[error("Given index was outside of chunk boundaries: {0}")]
pub struct InChunkIndexError(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Pod, Zeroable, Serialize, Deserialize)]
#[repr(transparent)]
/// A block position inside of a chunk, limited to 0..=[CHUNK_DIM]
pub struct InChunkPos(pub(crate) IVec3);

#[derive(Copy, Clone, PartialEq, Hash, Debug, Default, Pod, Zeroable, Serialize, Deserialize)]
#[repr(C)]
/// A range of block positions inside of a chunk, with coordinates limited to 0..[CHUNK_DIM] (min&max are *inclusive*)
pub struct InChunkRange {
    pub(crate) min: InChunkPos,
    pub(crate) max: InChunkPos,
}

impl Eq for InChunkRange {}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Pod, Zeroable, Serialize, Deserialize)]
#[repr(transparent)]
/// An absolute chunk position in a voxel world
pub struct AbsChunkPos(pub(crate) IVec3);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Pod, Zeroable, Serialize, Deserialize)]
#[repr(transparent)]
/// A chunk position relative to another chunk position
pub struct RelChunkPos(pub(crate) IVec3);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Pod, Zeroable, Serialize, Deserialize)]
#[repr(transparent)]
/// An absolute block position in a voxel world
pub struct AbsBlockPos(pub(crate) IVec3);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Pod, Zeroable, Serialize, Deserialize)]
#[repr(transparent)]
/// A block position relative to another block position
pub struct RelBlockPos(pub(crate) IVec3);

// === Utils
macro_rules! impl_simple_ivec3_newtype {
    ($T:ident) => {
        impl From<IVec3> for $T {
            fn from(value: IVec3) -> Self {
                Self(value)
            }
        }
        impl From<$T> for IVec3 {
            fn from(value: $T) -> IVec3 {
                value.0
            }
        }
        impl Deref for $T {
            type Target = IVec3;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

// === InChunkPos

impl TryFrom<IVec3> for InChunkPos {
    type Error = InChunkVecError;

    #[inline]
    fn try_from(value: IVec3) -> Result<Self, Self::Error> {
        if (value.cmplt(IVec3::ZERO) | value.cmpge(CHUNK_DIM3V)).any() {
            Err(InChunkVecError(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<InChunkPos> for IVec3 {
    #[inline]
    fn from(value: InChunkPos) -> IVec3 {
        value.0
    }
}

impl Deref for InChunkPos {
    type Target = IVec3;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl InChunkPos {
    /// Convert a XZY-strided index into a chunk storage array into the coordinates
    pub fn try_from_index(idx: usize) -> Result<Self, InChunkIndexError> {
        if idx >= CHUNK_DIM3Z {
            return Err(InChunkIndexError(idx));
        }
        let i: i32 = idx as i32;
        Ok(InChunkPos(IVec3::new(
            i % CHUNK_DIM,
            (i / CHUNK_DIM2) % CHUNK_DIM,
            (i / CHUNK_DIM) % CHUNK_DIM,
        )))
    }

    /// Converts the coordinates into an XZY-strided index into the chunk storage array
    pub fn as_index(self) -> usize {
        (self.x + (CHUNK_DIM * self.z) + (CHUNK_DIM2 * self.y)) as usize
    }
}

impl Add<InChunkPos> for InChunkPos {
    type Output = RelBlockPos;
    #[inline]
    fn add(self, rhs: InChunkPos) -> Self::Output {
        RelBlockPos(self.0 + rhs.0)
    }
}

// === InChunkRange
impl InChunkRange {
    pub fn from_corners(a: InChunkPos, b: InChunkPos) -> Self {
        let min = InChunkPos(a.min(*b));
        let max = InChunkPos(a.max(*b));
        Self { min, max }
    }

    pub fn is_empty(self) -> bool {
        self.min.cmpeq(*self.max).any()
    }

    pub fn min(self) -> InChunkPos {
        self.min
    }

    pub fn max(self) -> InChunkPos {
        self.max
    }

    pub fn iter_xzy(self) -> impl Iterator<Item = InChunkPos> {
        itertools::iproduct!(
            self.min.y..=self.max.y,
            self.min.z..=self.max.z,
            self.min.x..=self.max.x
        )
        .map(|(y, z, x)| InChunkPos(IVec3::new(y, z, x)))
    }
}

// === AbsChunkPos
impl_simple_ivec3_newtype!(AbsChunkPos);
// === RelChunkPos
impl_simple_ivec3_newtype!(RelChunkPos);
// === AbsBlockPos
impl_simple_ivec3_newtype!(AbsBlockPos);
// === RelBlockPos
impl_simple_ivec3_newtype!(RelBlockPos);
