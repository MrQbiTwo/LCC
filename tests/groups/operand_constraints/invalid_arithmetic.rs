// Invalid arithmetic cases for operand constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "reject_plus_pointer",
            "unary plus requires arithmetic type",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    return +p != 0;
}"#,
        ),
        (
            "reject_minus_pointer",
            "unary minus cannot negate a pointer",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    return -p != 0;
}"#,
        ),
        (
            "reject_plus_void",
            "unary plus rejects void values",
            r#"
int main(void) { return +(void)0; }"#,
        ),
        (
            "reject_minus_record",
            "unary minus rejects complete record values",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s = {3};
    return -s;
}"#,
        ),
        (
            "reject_complement_void",
            "bitwise complement requires integer type",
            r#"
void f(void) {}
int main(void) { return ~f(); }"#,
        ),
        (
            "reject_complement_float",
            "bitwise complement rejects floating type",
            r#"
int main(void) { return ~1.5; }"#,
        ),
        (
            "reject_complement_pointer",
            "bitwise complement rejects pointer type",
            r#"
int main(void) {
    int n = 3;
    return ~&n;
}"#,
        ),
        (
            "reject_plus_array",
            "array decay does not make unary plus arithmetic",
            r#"
int main(void) {
    int a[2] = {3, 5};
    return +a != 0;
}"#,
        ),
        (
            "reject_minus_function",
            "function decay does not make unary minus arithmetic",
            r#"
int f(void) { return 3; }
int main(void) { return -f != 0; }"#,
        ),
        (
            "reject_multiply_pointer",
            "multiplication requires arithmetic operands",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    return p * 2;
}"#,
        ),
        (
            "reject_divide_pointer",
            "division validates the right operand before common-type selection",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    return 2 / p;
}"#,
        ),
        (
            "reject_multiply_void",
            "multiplication rejects a void expression",
            r#"
void f(void) {}
int main(void) { return 2 * f(); }"#,
        ),
        (
            "reject_divide_union",
            "division rejects complete union values",
            r#"
union U {
    int n;
};
int main(void) {
    union U u = {3};
    return u / 2;
}"#,
        ),
        (
            "reject_mod_float",
            "remainder requires two integer operands",
            r#"
int main(void) { return 5 % 2.0; }"#,
        ),
        (
            "reject_bitand_pointer",
            "bitwise-and requires integer operands",
            r#"
int main(void) {
    int n = 3;
    return &n & 1;
}"#,
        ),
        (
            "reject_bitor_float",
            "bitwise-or rejects floating operands",
            r#"
int main(void) { return 1.5 | 2; }"#,
        ),
        (
            "reject_bitxor_pointer",
            "bitwise-xor checks the right operand's integer type",
            r#"
int main(void) {
    int n = 3;
    return 1 ^ &n;
}"#,
        ),
        (
            "reject_shift_left_float",
            "left shift requires an integer left operand",
            r#"
int main(void) { return 1.5 << 2; }"#,
        ),
        (
            "reject_shift_right_pointer",
            "right shift requires an integer count",
            r#"
int main(void) {
    int n = 3;
    return 8 >> &n;
}"#,
        ),
        (
            "reject_mod_void",
            "remainder rejects void even when the other operand is integer",
            r#"
int main(void) { return 3 % (void)0; }"#,
        ),
        (
            "reject_compound_multiply_pointer",
            "compound multiplication requires arithmetic operands",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    p *= 2;
    return 0;
}"#,
        ),
        (
            "reject_compound_divide_pointer",
            "compound division checks the right operand's arithmetic type",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    n /= p;
    return 0;
}"#,
        ),
        (
            "reject_compound_mod_float",
            "compound remainder requires integer operands",
            r#"
int main(void) {
    int n = 3;
    n %= 2.0;
    return 0;
}"#,
        ),
        (
            "reject_compound_bitand_pointer",
            "compound bitwise-and cannot modify a pointer",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    p &= 1;
    return 0;
}"#,
        ),
        (
            "reject_compound_bitor_float",
            "compound bitwise-or requires an integer left operand",
            r#"
int main(void) {
    double d = 1.5;
    d |= 2;
    return 0;
}"#,
        ),
        (
            "reject_compound_bitxor_void",
            "compound bitwise-xor rejects void on the right",
            r#"
int main(void) {
    int n = 3;
    n ^= (void)0;
    return 0;
}"#,
        ),
        (
            "reject_compound_shift_left_pointer",
            "compound left shift requires an integer destination",
            r#"
int main(void) {
    int n = 3;
    int *p = &n;
    p <<= 1;
    return 0;
}"#,
        ),
        (
            "reject_compound_shift_right_float",
            "compound right shift requires an integer count",
            r#"
int main(void) {
    int n = 3;
    n >>= 1.5;
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
