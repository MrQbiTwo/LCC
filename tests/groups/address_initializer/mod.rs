// Static address constants exercise symbol identity, byte addends and rejection.
use crate::Case;

// Verify each accepted form through both native ELF and independently assembled NASM.
mod constraints;
mod diagnostic_locations;
mod objects;
mod relocation_boundaries;
mod strings_and_expressions;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    objects::append(&mut cases);
    strings_and_expressions::append(&mut cases);
    relocation_boundaries::append(&mut cases);
    constraints::append(&mut cases);
    diagnostic_locations::append(&mut cases);
    cases
}
