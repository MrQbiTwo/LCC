// File-scope tentative record objects acquire storage after tag completion.
use crate::Case;

mod addresses_and_layout;
mod complete_values;
mod definitions;
mod incomplete_definitions;
mod incomplete_values;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    definitions::append(&mut cases);
    addresses_and_layout::append(&mut cases);
    complete_values::append(&mut cases);
    incomplete_definitions::append(&mut cases);
    incomplete_values::append(&mut cases);
    cases
}
