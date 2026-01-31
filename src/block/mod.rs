//! Composite form blocks.

mod address;
mod contact;
mod date_range;

pub use address::AddressBlock;
pub use contact::ContactBlock;
pub use date_range::DateRangeBlock;

use crate::field::Field;

/// Trait for composite form blocks that contain multiple fields.
pub trait Block: Send + Sync {
    /// Returns the block's prefix for field IDs.
    fn prefix(&self) -> &str;

    /// Returns all fields in this block.
    fn fields(&self) -> Vec<Box<dyn Field>>;

    /// Returns the block's title/label.
    fn title(&self) -> Option<&str> {
        None
    }
}
