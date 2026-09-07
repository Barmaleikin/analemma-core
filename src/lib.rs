//! # analemma
//!
//! Architecture-neutral balanced-ternary core library: trit arithmetic,
//! ternary words of configurable width, and sparse ternary arrays with
//! negative indices.
//!
//! ## Word width
//!
//! Selected at build time via `ANALEMMA_TRITS` (default `6`):
//!
//! | Width | Values | Table |
//! |-------|--------|-------|
//! | 3 | 27 | `static table::TABLE` |
//! | 6 | 729 | `static table::TABLE` |
//! | 9 | 19_683 | `static table::TABLE` |
//! | 18 | 387_420_489 | composite, no static table (`table::table6` base) |
//!
//! ## Layering
//!
//! This crate is the *core* only: numbers, trits, representations.
//! It knows nothing about instruction sets, punched tape or charsets.
//! Historical architectures (Setun-58, Setun-70) are supported by
//! separate crates that depend on this one.
//!
//! ## String representations
//!
//! `table::Entry` (3/6/9 trits) always carries pre-rendered `&'static str`
//! fields (`str_trits`, `str_nonary`, binary/decimal/hex forms) — they are
//! the primary debugging instrument. For 18 trits no static table exists,
//! so `Entry` implements `Display` built on demand; per-tryte strings are
//! reachable via `entry.trytes.*`.

#![allow(unexpected_cfgs)]

#[path = "table_generated.rs"]
pub mod table;

pub mod trit;
pub mod dword;
pub mod ternary_array;
pub mod tryte;
pub mod word;

#[cfg(test)]
mod tests;

/// Deprecated compatibility facade (0.1.x API).
///
/// Use [`table`] instead. ISA decoding, tape and charset support now live in
/// architecture crates on top of this core.
#[deprecated(
    since = "0.2.0",
    note = "use `analemma::table`; ISA/tape/charset moved to architecture crates"
)]
pub mod tryte_table {
    #![allow(deprecated)]

    pub use crate::table::entry_by_trits as tryte_entry_by_trits;
    pub use crate::table::entry_by_value as tryte_entry_by_value;
    pub use crate::table::Entry as TryteEntry;
    pub use crate::table::OFFSET as TRYTE_OFFSET;
    pub use crate::table::TABLE_SIZE as TRYTE_TABLE_SIZE;

    /// The core carries no instruction set; see architecture crates.
    pub const ISA_NAME: &str = "none";

    /// Only available when the core width is 6 trits (the tryte).
    #[cfg(analemma_trits = "6")]
    pub use crate::table::TABLE as TRYTE_TABLE;
}
