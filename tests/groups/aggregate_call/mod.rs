// Value calls verify aggregate register classes, copies, and result lifetimes.
use crate::Case;

mod arguments;
mod lifetime;
mod results;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    arguments::append(&mut cases);
    results::append(&mut cases);
    lifetime::append(&mut cases);
    cases
}
