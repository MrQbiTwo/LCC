// Cross-compiler cases use C endpoints on opposite sides of one ABI boundary.
use crate::{Case, Expectation};

// Swap the producer and consumer compiler while retaining one semantic oracle.
fn pair(cases: &mut Vec<Case>, name: &str, decl: &str, signature: &str, body: &str, driver: &str) {
    let callee = format!("#include <stdarg.h>\n{decl}{signature}{{{body}}}");
    let caller = format!("#include <stdarg.h>\n{decl}{signature};int driver(void){{{driver}}}");
    for (direction, source, foreign) in [
        ("in", callee.clone(), caller.clone()),
        ("out", caller, callee),
    ] {
        cases.push(Case::new(
            format!("interop_{name}_{direction}"),
            format!("foreign compiler boundary {name} {direction}"),
            source,
            Expectation::CInterop(foreign),
        ));
    }
}

// Cover value and list boundaries with distinct types and allocation failures.
mod inferred_arrays;
mod shared_lists;
mod values_and_calls;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    values_and_calls::append(&mut cases);
    shared_lists::append(&mut cases);
    inferred_arrays::append(&mut cases);
    cases
}
