// Invalid arguments cases for conversion constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "argument_nonzero",
            "fixed pointer parameters require null constants even in unevaluated calls",
            r#"int read_value(const int *pointer) {
    return *pointer;
}
int main(void) {
    return sizeof(read_value(1));
}
"#,
        ),
        (
            "argument_variable_zero",
            "fixed pointer arguments cannot use a zero-valued integer object",
            r#"int empty(const int *pointer) {
    return pointer == 0;
}
int main(void) {
    int zero = 0;
    return empty(zero);
}
"#,
        ),
        (
            "argument_float_zero",
            "fixed pointer arguments cannot use floating zero",
            r#"int empty(const int *pointer) {
    return pointer == 0;
}
int main(void) {
    if (0) {
        return empty(0.0);
    }
    return 0;
}
"#,
        ),
        (
            "argument_pointer_integer",
            "fixed arithmetic arguments cannot implicitly consume pointers",
            r#"int whole(long value) {
    return (int)value;
}
int main(void) {
    int value = 0;
    return sizeof(whole(&value));
}
"#,
        ),
        (
            "argument_discard_const",
            "fixed pointer arguments cannot discard pointee const",
            r#"int read_value(int *pointer) {
    return *pointer;
}
int main(void) {
    const int value = 0;
    return read_value(&value);
}
"#,
        ),
        (
            "argument_nested_qualifier",
            "fixed pointer arguments preserve nested qualifier compatibility",
            r#"int empty(const int **pointer) {
    return pointer == 0;
}
int main(void) {
    int *pointer = 0;
    return empty(&pointer);
}
"#,
        ),
        (
            "argument_function_type",
            "fixed function pointer arguments must have compatible parameter types",
            r#"int identity(long value) {
    return (int)value;
}
int invoke(int (*function)(int)) {
    return function(3);
}
int main(void) {
    return invoke(identity);
}
"#,
        ),
        (
            "argument_record_value",
            "fixed record arguments must have the parameter's canonical type",
            r#"struct Left { int value; };
struct Right { int value; };
int read_value(struct Left value) {
    return value.value;
}
int main(void) {
    struct Right value = {0};
    return read_value(value);
}
"#,
        ),
        (
            "argument_void_fixed",
            "a void expression cannot supply a fixed argument",
            r#"void nothing(void) {}
int identity(int value) {
    return value;
}
int main(void) {
    return sizeof(identity(nothing()));
}
"#,
        ),
        (
            "argument_void_variadic",
            "the variadic portion of a call cannot contain void expressions",
            r#"void nothing(void) {}
int first(int value, ...) {
    return value;
}
int main(void) {
    return sizeof(first(1, nothing()));
}
"#,
        ),
        (
            "argument_void_unspecified",
            "an unspecified parameter list still excludes void arguments",
            r#"void nothing(void) {}
int first();
int invoke(void) {
    return sizeof(first(nothing()));
}
int first(void) {
    return 0;
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
