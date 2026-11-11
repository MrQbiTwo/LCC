// Target header contracts and constant-expression member offsets.
use crate::{Case, Expectation};

fn pass(id: &str, purpose: &str, headers: &str, source: &str) -> Case {
    Case::new(
        format!("standard_headers_{id}"),
        purpose,
        format!("{headers}\n{source}"),
        Expectation::Exit(0),
    )
}

mod floating;
mod limits;
mod offset_constants;
mod offset_constraints;
mod offset_layout;
mod stddef;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    stddef::append(&mut cases);
    limits::append(&mut cases);
    floating::append(&mut cases);
    offset_layout::append(&mut cases);
    offset_constants::append(&mut cases);
    offset_constraints::append(&mut cases);
    cases
}
