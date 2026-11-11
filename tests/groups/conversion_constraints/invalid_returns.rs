// Invalid returns cases for conversion constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "return_nonzero",
            "a pointer result cannot implicitly return a nonzero integer",
            r#"int *pointer(void) {
    return 1;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_variable_zero",
            "a zero-valued local is not a null pointer constant in a return",
            r#"int *pointer(void) {
    int zero = 0;
    return zero;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_float_zero",
            "a pointer result cannot implicitly return floating zero",
            r#"int *pointer(void) {
    return 0.0;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_pointer_integer",
            "an arithmetic result cannot implicitly return an object pointer",
            r#"static int value;
long address(void) {
    return &value;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_discard_const",
            "pointer results cannot discard immediate pointee const",
            r#"int *pointer(const int *value) {
    return value;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_nested_qualifier",
            "pointer return compatibility preserves nested pointee qualifiers",
            r#"const int **pointer(int **value) {
    return value;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_record_value",
            "record return expressions require the declared canonical record type",
            r#"struct Left { int value; };
struct Right { int value; };
struct Left value(void) {
    struct Right result = {0};
    return result;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_void_value",
            "a nonvoid function cannot return a void expression",
            r#"void nothing(void) {}
int value(void) {
    return nothing();
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_expression_from_void",
            "a C89 void function cannot return a nonvoid expression",
            r#"void nothing(void) {
    return 3;
}
int main(void) {
    return 0;
}
"#,
        ),
        (
            "return_void_expression_from_void",
            "a C89 void function cannot return even a void-valued expression",
            r#"void nothing(void) {}
void another(void) {
    return nothing();
}
int main(void) {
    return 0;
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("conversion_reject_{suffix}"),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
