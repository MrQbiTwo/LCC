// Explicit casts validate their own types without imposing implicit conversions.
use crate::Case;

mod accepted;
mod constraints;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    accepted::append(&mut cases);
    constraints::append(&mut cases);
    cases
}
