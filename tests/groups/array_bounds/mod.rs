// Unknown outer array bounds cover ownership, completion, layout and diagnostics.
use crate::Case;

// Run storage-sensitive examples as both automatic and file-scope objects.
mod constraints;
mod declarations;
mod inferred_objects;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    inferred_objects::append(&mut cases);
    declarations::append(&mut cases);
    constraints::append(&mut cases);
    cases
}
