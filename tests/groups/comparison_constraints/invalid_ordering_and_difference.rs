// Invalid ordering and difference cases for comparison constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (suffix, purpose, declarations, expression) in [
        (
            "relational_integer",
            "relational pointer operands cannot use an integer null constant",
            r#"int *pointer;"#,
            "pointer < 0",
        ),
        (
            "relational_integer_reverse",
            "reversed relational pointer operands cannot use integer constants",
            r#"int *pointer;"#,
            "0 >= pointer",
        ),
        (
            "relational_floating",
            "relational operands cannot mix pointers and floating values",
            r#"int *pointer;"#,
            "pointer > 0.0",
        ),
        (
            "relational_void",
            "void values are not relational operands",
            r#"void nothing(void) {}"#,
            "nothing() <= nothing()",
        ),
        (
            "relational_record",
            "record values are not relational operands",
            r#"struct Record { int value; };
struct Record object;"#,
            "object >= object",
        ),
        (
            "relational_function",
            "function pointers do not satisfy relational object or incomplete type constraints",
            r#"int (*left)(int);
int (*right)(int);"#,
            "left < right",
        ),
        (
            "relational_void_object",
            "void and integer pointees are not compatible for relational comparisons",
            r#"void *left;
int *right;"#,
            "left <= right",
        ),
        (
            "relational_void_null",
            "relational comparison has no special null pointer constant exception",
            r#"int *pointer;"#,
            "pointer >= (void *)0",
        ),
        (
            "relational_signedness",
            "relational pointer operands require compatible pointed-to integer types",
            r#"int *left;
unsigned int *right;"#,
            "left > right",
        ),
        (
            "relational_record_identity",
            "relational comparisons preserve incomplete record tag identity",
            r#"struct Left;
struct Right;
struct Left *left;
struct Right *right;"#,
            "left < right",
        ),
        (
            "relational_nested_qualifier",
            "relational compatibility preserves nested pointee qualifiers",
            r#"int **left;
const int **right;"#,
            "left >= right",
        ),
        (
            "relational_array_completeness",
            "relational array pointees must both be complete or both incomplete",
            r#"int (*left)[];
int (*right)[3];"#,
            "left < right",
        ),
        (
            "relational_array_completeness_reverse",
            "relational pointee completeness matching applies in either order",
            r#"int (*left)[3];
int (*right)[];"#,
            "left >= right",
        ),
        (
            "difference_signedness",
            "pointer differences require compatible signedness",
            r#"int *left;
unsigned int *right;"#,
            "left - right",
        ),
        (
            "difference_record_identity",
            "equal-sized distinct records cannot be subtracted through pointers",
            r#"struct Left { int value; };
struct Right { int value; };
struct Left *left;
struct Right *right;"#,
            "left - right",
        ),
        (
            "difference_nested_qualifier",
            "pointer differences preserve nested pointee qualifiers",
            r#"int **left;
const int **right;"#,
            "left - right",
        ),
        (
            "difference_nested_type",
            "equal-sized pointer pointees must still be compatible",
            r#"int **left;
long **right;"#,
            "left - right",
        ),
        (
            "difference_array_bound",
            "pointer differences require compatible complete array bounds",
            r#"int (*left)[2];
int (*right)[3];"#,
            "left - right",
        ),
        (
            "difference_incomplete_record",
            "incomplete record pointees have no subtraction element size",
            r#"struct Hidden;
struct Hidden *left;
struct Hidden *right;"#,
            "left - right",
        ),
        (
            "difference_unknown_array",
            "an unknown array bound does not provide a subtraction element size",
            r#"int (*left)[];
int (*right)[];"#,
            "left - right",
        ),
        (
            "difference_void",
            "void pointees have no object size for pointer subtraction",
            r#"void *left;
void *right;"#,
            "left - right",
        ),
        (
            "difference_function",
            "function pointees are not objects for pointer subtraction",
            r#"int (*left)(int);
int (*right)(int);"#,
            "left - right",
        ),
    ] {
        cases.push(Case::new(
            format!("comparison_reject_{suffix}"),
            purpose,
            format!("{declarations}\nint main(void) {{\n    return sizeof({expression});\n}}\n"),
            Expectation::Reject("E008".into()),
        ));
    }
}
