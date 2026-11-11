// Assignment-like contexts share conversion rules before code generation.
use crate::Case;

mod arguments_and_returns;
mod assignment_and_initialization;
mod invalid_arguments;
mod invalid_assignments;
mod invalid_initializers;
mod invalid_returns;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    assignment_and_initialization::append(&mut cases);
    arguments_and_returns::append(&mut cases);
    invalid_assignments::append(&mut cases);
    invalid_initializers::append(&mut cases);
    invalid_arguments::append(&mut cases);
    invalid_returns::append(&mut cases);
    cases
}
