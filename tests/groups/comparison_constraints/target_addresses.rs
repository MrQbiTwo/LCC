// Target addresses cases for comparison constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Target representation probes: these synthetic function addresses are
    // never called. They are deliberately outside portable C oracle claims.
    cases.push(Case::new(
        "comparison_target_high_function_zero",
        "LP64 function designator equality retains bits above the low 32 bits",
        r#"int main(void) {
    return *(int (*)(void))4294967296UL == 0;
}
"#,
        Expectation::Exit(0),
    ));
    cases.push(Case::new(
        "comparison_target_high_function_pair",
        "LP64 function designators with equal low words remain distinct",
        r#"int main(void) {
    return *(int (*)(void))4294967297UL == *(int (*)(void))1UL;
}
"#,
        Expectation::Exit(0),
    ));
}
