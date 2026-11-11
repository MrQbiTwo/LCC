// Constraints cases for conditional composite.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, source) in [
        (
            "array_bound",
            "different known array bounds remain incompatible conditional operands",
            r#"int main(void) {
    int (*left)[2] = 0;
    int (*right)[3] = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
        (
            "nested_array_bound",
            "recursive composition rejects conflicting bounds below nested pointers",
            r#"int main(void) {
    int (**left)[2] = 0;
    int (**right)[3] = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
        (
            "nested_qualifier",
            "conditional composition cannot add qualifiers beneath the immediate pointee",
            r#"int main(void) {
    int **left = 0;
    const int **right = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
        (
            "array_source_incomplete",
            "a completed conditional result does not complete its original unknown-array operand",
            r#"int main(void) {
    int (*unknown)[] = 0;
    int (*known)[3] = 0;
    (void)sizeof(*(1 ? unknown : known));
    return sizeof(*unknown);
}
"#,
        ),
        (
            "learned_prototype_too_few",
            "a prototype learned from the right operand enforces its minimum argument count",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(int) = 0;
    return sizeof((1 ? unspecified : prototype)());
}
"#,
        ),
        (
            "learned_prototype_too_many",
            "a prototype learned from the left operand rejects excess arguments",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(int) = 0;
    return sizeof((0 ? prototype : unspecified)(3, 5));
}
"#,
        ),
        (
            "learned_void_prototype",
            "a learned empty prototype rejects arguments even in an unevaluated call",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(void) = 0;
    return sizeof((1 ? unspecified : prototype)(3));
}
"#,
        ),
        (
            "learned_prototype_argument_type",
            "a learned fixed pointer parameter retains the implicit conversion constraints",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(int *) = 0;
    return sizeof((1 ? unspecified : prototype)(3));
}
"#,
        ),
        (
            "learned_parameter_array_bound",
            "a composite function parameter rejects an array with the wrong known bound",
            r#"int main(void) {
    int values[2] = {3, 5};
    int (*unknown)(int (*)[]) = 0;
    int (*known)(int (*)[3]) = 0;
    return sizeof((1 ? unknown : known)(&values));
}
"#,
        ),
        (
            "learned_second_parameter_bound",
            "bounds learned from the other arm also constrain later function parameters",
            r#"int main(void) {
    int first[3] = {2, 3, 5};
    long second[3] = {7, 11, 13};
    int (*left)(int (*)[], long (*)[2]) = 0;
    int (*right)(int (*)[3], long (*)[]) = 0;
    return sizeof((0 ? left : right)(&first, &second));
}
"#,
        ),
        (
            "unspecified_char_parameter",
            "char changes under default promotions and cannot compose with an unspecified signature",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(char) = 0;
    return sizeof(1 ? unspecified : prototype);
}
"#,
        ),
        (
            "unspecified_float_parameter",
            "float changes under default promotions and cannot compose with an unspecified signature",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(float) = 0;
    return sizeof(1 ? prototype : unspecified);
}
"#,
        ),
        (
            "unspecified_variadic",
            "an unspecified parameter list is incompatible with a variadic prototype",
            r#"int main(void) {
    int (*unspecified)() = 0;
    int (*prototype)(int, ...) = 0;
    return sizeof(1 ? unspecified : prototype);
}
"#,
        ),
        (
            "fixed_variadic",
            "fixed and variadic prototypes remain incompatible",
            r#"int main(void) {
    int (*fixed)(int) = 0;
    int (*variadic)(int, ...) = 0;
    return sizeof(1 ? fixed : variadic);
}
"#,
        ),
        (
            "function_parameter_nested_qualifier",
            "ignoring top parameter qualifiers must preserve nested pointee qualifiers",
            r#"int main(void) {
    int (*plain)(int *) = 0;
    int (*qualified)(const int *) = 0;
    return sizeof(1 ? plain : qualified);
}
"#,
        ),
        (
            "function_return_array_bound",
            "recursive function result composition rejects conflicting array bounds",
            r#"typedef int First[2];
typedef int Second[3];
int main(void) {
    First *(*left)(void) = 0;
    Second *(*right)(void) = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
        (
            "record_identity",
            "recursive composition does not merge separate record tags",
            r#"struct First { int value; };
struct Second { int value; };
int main(void) {
    struct First **left = 0;
    struct Second **right = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
        (
            "enum_identity",
            "recursive composition does not merge separate enum tags",
            r#"enum First { FIRST_NEGATIVE = -1, FIRST = 3 };
enum Second { SECOND_NEGATIVE = -1, SECOND = 5 };
int main(void) {
    enum First **left = 0;
    enum Second **right = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
        (
            "char_identity",
            "recursive composition preserves the distinction between plain and signed char",
            r#"int main(void) {
    char **left = 0;
    signed char **right = 0;
    return sizeof(1 ? left : right);
}
"#,
        ),
    ] {
        cases.push(Case::new(
            format!("conditional_composite_reject_{suffix}"),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
