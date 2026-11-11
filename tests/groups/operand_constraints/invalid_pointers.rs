// Invalid pointers cases for operand constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "reject_subscript_float",
            "a subscript index must have integer type",
            r#"
int main(void) {
    int a[2] = {3, 5};
    return a[1.0];
}"#,
        ),
        (
            "reject_subscript_pointer",
            "two pointer operands cannot form an array subscript",
            r#"
int main(void) {
    int a[2] = {3, 5};
    int *p = a;
    return a[p];
}"#,
        ),
        (
            "reject_subscript_void",
            "a void expression cannot be a subscript index",
            r#"
void f(void) {}
int main(void) {
    int a[2] = {3, 5};
    return a[f()];
}"#,
        ),
        (
            "reject_subscript_record",
            "a complete record cannot be a subscript index",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {1};
    int a[2] = {3, 5};
    return a[s];
}"#,
        ),
        (
            "reject_subscript_integers",
            "subscripting needs one pointer operand as well as an integer",
            r#"
int main(void) {
    int n = 3;
    return n[1];
}"#,
        ),
        (
            "reject_sizeof_unary_pointer",
            "unevaluated sizeof does not waive unary operand constraints",
            r#"
int main(void) {
    int n = 3;
    return sizeof(+&n);
}"#,
        ),
        (
            "reject_sizeof_void_logical",
            "unevaluated logical operators still require scalar operands",
            r#"
void f(void) {}
int main(void) { return sizeof(0 && f()); }"#,
        ),
        (
            "reject_add_void",
            "addition rejects a void operand",
            r#"
void f(void) {}
int main(void) { return 1 + f(); }"#,
        ),
        (
            "reject_subtract_void",
            "subtraction rejects a void left operand",
            r#"
int main(void) { return (void)0 - 1; }"#,
        ),
        (
            "reject_add_pointers",
            "two pointers cannot be added",
            r#"
int main(void) {
    int a[2] = {3, 5};
    int *p = a;
    return p + a != 0;
}"#,
        ),
        (
            "reject_integer_minus_pointer",
            "subtraction cannot subtract a pointer from an integer",
            r#"
int main(void) {
    int n = 3;
    return 1 - &n;
}"#,
        ),
        (
            "reject_add_function",
            "function designators cannot participate in pointer arithmetic",
            r#"
int f(void) { return 3; }
int main(void) { return f + 1 != 0; }"#,
        ),
        (
            "reject_pointer_add_float",
            "pointer addition requires an integer offset",
            r#"
int main(void) {
    int a[2] = {3, 5};
    return a + 1.0 != 0;
}"#,
        ),
        (
            "reject_compound_add_void",
            "compound addition rejects void right operands",
            r#"
int main(void) {
    int n = 3;
    n += (void)0;
    return 0;
}"#,
        ),
        (
            "reject_compound_subtract_pointers",
            "compound pointer subtraction cannot assign an integer difference",
            r#"
int main(void) {
    int a[2] = {3, 5};
    int *p = a;
    p -= a;
    return 0;
}"#,
        ),
        (
            "reject_compound_add_incomplete",
            "compound pointer addition requires a complete element type",
            r#"
struct S;
int main(void) {
    struct S *p = 0;
    p += 1;
    return 0;
}"#,
        ),
        (
            "reject_compound_subtract_incomplete",
            "compound pointer subtraction requires a complete element type",
            r#"
struct S;
int main(void) {
    struct S *p = 0;
    p -= 1;
    return 0;
}"#,
        ),
        (
            "reject_compound_add_void_pointer",
            "void pointers have no object stride for compound addition",
            r#"
int main(void) {
    void *p = 0;
    p += 1;
    return 0;
}"#,
        ),
        (
            "reject_compound_subtract_function_pointer",
            "function pointers have no object stride for compound subtraction",
            r#"
int f(void) { return 3; }
int main(void) {
    int (*p)(void) = f;
    p -= 1;
    return 0;
}"#,
        ),
        (
            "reject_compound_add_pointer_float",
            "compound pointer addition requires an integer offset",
            r#"
int main(void) {
    int a[2] = {3, 5};
    int *p = a;
    p += 1.0;
    return 0;
}"#,
        ),
        (
            "reject_subscript_void_pointer",
            "subscripting requires an object element type",
            r#"
int main(void) {
    void *p = 0;
    p[0];
    return 0;
}"#,
        ),
        (
            "reject_subscript_incomplete_pointer",
            "subscripting requires a complete object element type",
            r#"
struct S;
int main(void) {
    struct S *p = 0;
    p[0];
    return 0;
}"#,
        ),
        (
            "reject_reverse_subscript_function_pointer",
            "reversed subscripting rejects function pointer element types",
            r#"
int f(void) { return 3; }
int main(void) {
    int (*p)(void) = f;
    return 0 [p]();
}"#,
        ),
        (
            "reject_reverse_subscript_float",
            "reversed subscripting still requires an integer index",
            r#"
int main(void) {
    int a[2] = {3, 5};
    return 1.0 [a];
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
