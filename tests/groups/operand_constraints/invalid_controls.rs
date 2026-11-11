// Invalid controls cases for operand constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "reject_if_record",
            "if requires a scalar controlling expression",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {0};
    if (s)
        return 1;
    return 0;
}"#,
        ),
        (
            "reject_if_void",
            "a void call cannot control if",
            r#"
void f(void) {}
int main(void) {
    if (f())
        return 1;
    return 0;
}"#,
        ),
        (
            "reject_while_union",
            "while requires a scalar controlling expression",
            r#"
union U {
    int n;
};
int main(void) {
    union U u = {0};
    while (u)
        break;
    return 0;
}"#,
        ),
        (
            "reject_do_void",
            "do-while requires a scalar controlling expression",
            r#"
int main(void) {
    do {
    } while ((void)0);
    return 0;
}"#,
        ),
        (
            "reject_for_record",
            "only the controlling for clause requires scalar type",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {0};
    for (; s;)
        break;
    return 0;
}"#,
        ),
        (
            "reject_for_void",
            "a void expression cannot be the for condition",
            r#"
void f(void) {}
int main(void) {
    for (; f();)
        break;
    return 0;
}"#,
        ),
        (
            "reject_conditional_record",
            "the first conditional operand requires scalar type",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {0};
    return s ? 1 : 0;
}"#,
        ),
        (
            "reject_conditional_void",
            "a void cast cannot select a conditional arm",
            r#"
int main(void) { return (void)0 ? 1 : 0; }"#,
        ),
        (
            "reject_not_void",
            "logical negation requires a scalar operand",
            r#"
void f(void) {}
int main(void) { return !f(); }"#,
        ),
        (
            "reject_not_record",
            "a complete record is not a scalar logical operand",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {0};
    return !s;
}"#,
        ),
        (
            "reject_and_record_left",
            "both logical-and operands must be scalar",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {0};
    return s && 1;
}"#,
        ),
        (
            "reject_and_void_right",
            "short-circuit evaluation does not waive right operand constraints",
            r#"
void f(void) {}
int main(void) { return 0 && f(); }"#,
        ),
        (
            "reject_or_void_left",
            "logical-or checks the left operand's scalar type",
            r#"
int main(void) { return (void)0 || 1; }"#,
        ),
        (
            "reject_or_union_right",
            "a skipped logical-or operand must still have scalar type",
            r#"
union U {
    int n;
};
int main(void) {
    union U u = {0};
    return 1 || u;
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
