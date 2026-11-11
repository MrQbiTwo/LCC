// Target aggregate layout, member addressing, value copies, and invalid uses.
use crate::Case;

mod constraints;
mod independent_layout;
mod values;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    values::append(&mut cases);
    constraints::append(&mut cases);
    independent_layout::append(&mut cases);
    cases
}
