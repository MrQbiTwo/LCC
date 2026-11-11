// Copy into groups/<feature>.rs and register it once in catalog.rs.
// This example module is deliberately unregistered and does not add coverage.
use crate::{Case, Expectation};

// Group files only describe inputs and independent expectations. The shared
// harness owns temporary files, subprocesses, deadlines, and output modes.
pub fn cases() -> Vec<Case> {
    let mut cases = vec![
        Case::new(
            "example_local_value",
            "an initialized local retains its value",
            r#"int main(void) {
    int value = 7;
    return value;
}
"#,
            Expectation::Exit(7),
        ),
        Case::new(
            "example_unknown_name",
            "an undeclared object produces the name-resolution diagnostic",
            r#"int main(void) {
    return missing;
}
"#,
            Expectation::Reject("E009".into()),
        ),
        Case::new(
            "example_local_header",
            "a quoted include resolves a case-local auxiliary header",
            r#"#include "value.h"
int main(void) {
    return VALUE;
}
"#,
            Expectation::Exit(11),
        )
        .with_files(vec![("value.h", "#define VALUE 11\n")]),
    ];

    // Use named columns for closely related boundaries, with a stable suffix
    // and an independently specified result for each row. Prefer descriptive
    // IDs over list positions: inserting a row must not rename existing cases.
    for (suffix, expression, expected_exit) in [
        ("zero", "0", 0),
        ("positive", "17", 17),
        ("negative", "-1", 255),
    ] {
        cases.push(Case::new(
            format!("example_integer_{suffix}"),
            format!("integer return boundary: {suffix}"),
            format!("int main(void) {{\n    return {expression};\n}}\n"),
            Expectation::Exit(expected_exit),
        ));
    }

    // Other expectation variants cover exact I/O, expected timeouts, NASM ABI
    // probes, foreign C endpoints, decimal conversion, and command-line output.
    // Keep complex C/NASM inputs in readable multiline strings or fixtures.
    // Preserve exact bytes for preprocessing, source-origin, and transcript tests.
    cases
}
