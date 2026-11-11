// Conditional pointer results retain the composite type of compatible operands.
use crate::Case;

mod compatible_pointers;
mod constraints;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    compatible_pointers::append(&mut cases);
    constraints::append(&mut cases);
    cases
}
