// Storage restrictions follow declaration context and the addressed object.
use crate::Case;

mod constraints;
mod register_objects;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    register_objects::append(&mut cases);
    constraints::append(&mut cases);
    cases
}
