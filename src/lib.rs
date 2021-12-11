//! Mini Ram Transactional Store is a simple transactional store designed to survive device resets
//!
//! # Assumptions
//!
//! 1. data is stored in a ram mapped region, which can be written and read using normal memory
//!    accesses
//! 2. atomic writes to this memory region are actually written atomically to the backing store
//!    (ie: a cache/bus, if it exists, does not generate 2 bus accesses from a single atomic write
//!    instruction)
//! 3. 32-bit wide atomic writes are supported for this memory region
//! 4. 8-bit reads are supported for this memory region
//!
//!
//! # Design
//!
//! - each entry has a 32-bit "marker", which includes a checksum, key (12-bit id), and some bits
//!   used for accounting and transactions.
//! - each entry marker has a "priority" which is used for picking between entries with identical
//!   ids.
//!
//!
//! # Limitations
//!
//! - 12-bits is not a lot of space to encode identifiers. Consider in future designs allowing
//!   arbitrary key sizes by encoding them as variable length bytes
//! - very limited priority system which does not support "log" style store. Consider in future
//!   designs providing "epoch" or "era" field on some or all entries.

#![no_std]

/// An instance of the store
pub struct Mrts<T> {
    data: T
}

/// Store global header magic identifying that this is actually a Mrts
const MAGIC_V1: u32 = 0xf5d97e41;
/// Maximum ID for any element in the store (12-bits)
const MAX_ID: u16 = 1 << 12 - 1;

/// number of bytes the can be covered by a single empty element
///
/// 4 bytes for it's mark, plus up to 255 4-byte blocks 
const MAX_BYTES_PER_EMPTY = 4 + (4 * 0xff);

/// The header present at the start of a `Mrts`
#[repr(C)]
struct StoreHeader {
    /// The value `MAGIC_V1`
    magic: u32,

    


    checksum: u32,
}

/// A marker present before each element of the store
struct ElemMark {
    // split into a few parts:
    //   kind: u4,
    //   id: u12,
    //   value_or_len: u8,
    //   checksum: u8
    mark: u32,
}

impl ElemMark {
    
}

impl<T: AsMut<[u8]>> Mrts<T> {
    pub fn from_bytes(data: T) -> Self {
        // Steps:
        // 1. determine if we're in a partially applied transaction, if so, complere application.
        //    If not, roll back an in-progress transaction
        // 2. 

        todo!()
    }
}
