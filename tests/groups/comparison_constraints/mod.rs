// Comparisons validate compatible operands even when their values are unevaluated.
use crate::Case;

mod compatible_values;
mod invalid_equality;
mod invalid_ordering_and_difference;
mod target_addresses;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    compatible_values::append(&mut cases);
    invalid_equality::append(&mut cases);
    invalid_ordering_and_difference::append(&mut cases);
    target_addresses::append(&mut cases);
    cases
}
