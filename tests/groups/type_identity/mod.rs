// Type identity is separate from the arithmetic representation of a value.
use crate::Case;

mod compatible_types;
mod distinct_characters;
mod distinct_enums;
mod target_policy;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    compatible_types::append(&mut cases);
    distinct_enums::append(&mut cases);
    distinct_characters::append(&mut cases);
    target_policy::append(&mut cases);
    cases
}
