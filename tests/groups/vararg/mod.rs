// Typed traversal exercises independent banks, overflow alignment, and list copies.
use crate::Case;

mod aggregate_allocation;
mod aggregate_fetch;
mod constraints;
mod list_lifetime;
mod scalar_arguments;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    scalar_arguments::append(&mut cases);
    list_lifetime::append(&mut cases);
    aggregate_allocation::append(&mut cases);
    aggregate_fetch::append(&mut cases);
    constraints::append(&mut cases);
    cases
}
