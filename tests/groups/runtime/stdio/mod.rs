// End-to-end standard streams, exact formatting and scanner cursor regressions.
use crate::{Case, Expectation};

fn io(id: &str, purpose: &str, body: &str, input: &str, output: String) -> Case {
    Case::new(
        format!("stdio_{id}"),
        purpose,
        format!("#include <stdio.h>\nint main(void){{{body}}}"),
        Expectation::Io(input.into(), output, 0),
    )
}

mod buffering;
mod formatted_input;
mod formatted_output;
mod input_boundaries;
mod output_constraints;

// Compose focused case families without changing their IDs or order.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    formatted_output::append(&mut cases);
    output_constraints::append(&mut cases);
    formatted_input::append(&mut cases);
    input_boundaries::append(&mut cases);
    buffering::append(&mut cases);
    cases
}
