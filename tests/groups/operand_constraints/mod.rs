// Operator constraints preserve scalar decay and reject invalid operand kinds.
use crate::Case;

mod arithmetic_and_indexing;
mod invalid_arithmetic;
mod invalid_controls;
mod invalid_pointers;
mod invalid_void_results;
mod scalar_controls;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    scalar_controls::append(&mut cases);
    arithmetic_and_indexing::append(&mut cases);
    invalid_controls::append(&mut cases);
    invalid_arithmetic::append(&mut cases);
    invalid_pointers::append(&mut cases);
    invalid_void_results::append(&mut cases);
    cases
}
