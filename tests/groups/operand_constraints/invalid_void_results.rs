// Invalid void results cases for operand constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "reject_nested_void_control",
            "nested conditional results with void type cannot control a statement",
            r#"
void tick(void) {}
int main(void) {
    if (1 ? (0 ? tick() : (void)0) : tick())
        return 1;
    return 0;
}"#,
        ),
        (
            "reject_void_conditional_arithmetic",
            "a conditional with two void arms remains invalid in arithmetic",
            r#"
void tick(void) {}
int main(void) {
    return 1 + (1 ? tick() : (void)0);
}"#,
        ),
        (
            "reject_void_conditional_logical",
            "a conditional with two void arms remains invalid as a logical operand",
            r#"
void tick(void) {}
int main(void) {
    return (1 ? tick() : (void)0) && 1;
}"#,
        ),
        (
            "reject_void_conditional_index",
            "a conditional with two void arms cannot provide an integer subscript",
            r#"
void tick(void) {}
int main(void) {
    int a[2] = {3, 5};
    return a[1 ? tick() : (void)0];
}"#,
        ),
        (
            "reject_sizeof_void_conditional",
            "sizeof rejects the void result type of a conditional expression",
            r#"
void tick(void) {}
int main(void) {
    return sizeof(1 ? tick() : (void)0);
}"#,
        ),
        (
            "reject_conditional_void_then_value",
            "a void second conditional operand requires a void third operand",
            r#"
void tick(void) {}
int main(void) {
    1 ? tick() : 3;
    return 0;
}"#,
        ),
        (
            "reject_conditional_value_then_void",
            "a void third conditional operand requires a void second operand",
            r#"
void tick(void) {}
int main(void) {
    1 ? 3 : tick();
    return 0;
}"#,
        ),
    ] {
        cases.push(Case::new(
            format!("operand_{}", id),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
