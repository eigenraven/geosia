use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

use crate::voxeltypes::BlockId;

pub const CHUNK_DIM: i32 = 32;
pub const CHUNK_DIMZ: usize = CHUNK_DIM as usize;
pub const CHUNK_DIM2: i32 = CHUNK_DIM * CHUNK_DIM;
pub const CHUNK_DIM2Z: usize = (CHUNK_DIM * CHUNK_DIM) as usize;
pub const CHUNK_DIM3: i32 = CHUNK_DIM * CHUNK_DIM * CHUNK_DIM;
pub const CHUNK_DIM3Z: usize = (CHUNK_DIM * CHUNK_DIM * CHUNK_DIM) as usize;

#[derive(Clone, Eq, PartialEq)]
pub struct PaletteStorage<DataType, IndexType, const INLINE_PALETTE_SIZE: usize> {
    palette: SmallVec<[DataType; INLINE_PALETTE_SIZE]>,
    data: [IndexType; CHUNK_DIM3Z],
}

pub type ArrayStorage<T> = [T; CHUNK_DIM3Z];
pub type PaletteStorage16<T> = PaletteStorage<T, u8, 16>;
pub type PaletteStorage256<T> = PaletteStorage<T, u8, 64>;
pub type PaletteStorage32k<T> = PaletteStorage<T, u16, 1024>;

#[repr(transparent)]
#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Serialize, Deserialize)]
pub struct BlockLight(u16);

#[derive(Clone, Eq, PartialEq)]
pub enum PaletteData<T> {
    Singleton(T),
    Type16(Box<PaletteStorage16<T>>),
    Type256(Box<PaletteStorage256<T>>),
    Type32k(Box<PaletteStorage32k<T>>),
}

#[derive(Clone, Eq, PartialEq)]
pub enum ArrayData<T> {
    Singleton(T),
    Array(Box<ArrayStorage<T>>),
}

#[derive(Clone, Eq, PartialEq)]
pub struct Chunk {
    blocks: PaletteData<BlockId>,
    light_level: ArrayData<BlockLight>,
}
