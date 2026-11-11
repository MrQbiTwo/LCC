// Block declarations must separate lexical visibility from storage lifetime.
use crate::Case;

// Exercise persistent data and linked block aliases in both compiler outputs.
mod extern_bindings;
mod function_composition;
mod invalid_declarations;
mod static_lifetime;
mod static_values;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    static_lifetime::append(&mut cases);
    static_values::append(&mut cases);
    extern_bindings::append(&mut cases);
    invalid_declarations::append(&mut cases);
    function_composition::append(&mut cases);
    cases
}
