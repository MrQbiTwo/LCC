// Arithmetic and indexing cases for operand constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "unary_integer_promotions",
            "unary arithmetic and complement promote narrow integers",
            r#"
int main(void) {
    unsigned char c = 3;
    signed char s = -4;
    short n = 5;
    return !(+c == 3 && -s == 4 && ~c == -4 && +n == 5 && sizeof(+c) == sizeof(int) &&
             sizeof(~s) == sizeof(int));
}"#,
        ),
        (
            "unary_floating",
            "unary arithmetic preserves all supported floating formats",
            r#"
int main(void) {
    float f = 1.5f;
    double d = -2.25;
    long double l = 3.5L;
    return !(+f == 1.5f && -f == -1.5f && +d == -2.25 && -d == 2.25 && +l == 3.5L &&
             -l == -3.5L);
}"#,
        ),
        (
            "mixed_arithmetic",
            "multiplication and division retain usual arithmetic conversions",
            r#"
int main(void) {
    int n = 3;
    float f = 1.5f;
    double d = 6.0;
    long double l = 2.0L;
    return !(n * f == 4.5f && d / n == 2.0 && l * n == 6.0L && d / l == 3.0L);
}"#,
        ),
        (
            "integer_operations",
            "integer-only binary operators retain enum and narrow operands",
            r#"
enum E { THREE = 3 };
int main(void) {
    unsigned char c = 13;
    short s = 2;
    enum E e = THREE;
    return !(c % e == 1 && (c & e) == 1 && (c | e) == 15 && (c ^ e) == 14 &&
             (c << s) == 52 && (c >> s) == 3);
}"#,
        ),
        (
            "compound_arithmetic",
            "arithmetic compound assignments retain floating conversion behavior",
            r#"
int main(void) {
    float f = 1.5f;
    double d = 6.0;
    long double l = 2.0L;
    int n = 3;
    f *= 2;
    d /= 3;
    l *= 1.5L;
    n *= 2;
    n /= 3;
    return !(f == 3.0f && d == 2.0 && l == 3.0L && n == 2);
}"#,
        ),
        (
            "compound_integer",
            "integer compound operators retain assignment conversions",
            r#"
int main(void) {
    unsigned char n = 13;
    n %= 5;
    n |= 8;
    n &= 14;
    n ^= 3;
    n <<= 2;
    n >>= 1;
    return n != 18;
}"#,
        ),
        (
            "narrow_enum_subscript",
            "integer promotion permits narrow and enum subscripts",
            r#"
enum E { INDEX = 2 };
int main(void) {
    int a[3] = {3, 5, 7};
    signed char n = 1;
    enum E e = INDEX;
    int *p = a + 1;
    return !(a[n] == 5 && a[e] == 7 && p[-n] == 3);
}"#,
        ),
        (
            "bitfield_subscript",
            "integer bit fields remain valid subscript operands",
            r#"
struct S {
    unsigned n : 3;
};
int main(void) {
    struct S s = {2};
    int a[3] = {3, 5, 7};
    return a[s.n] != 7;
}"#,
        ),
        (
            "pointer_steps",
            "valid pointer increments and additive forms retain object scaling",
            r#"
int main(void) {
    int a[4] = {3, 5, 7, 11};
    int *p = a;
    p++;
    ++p;
    p -= 1;
    p += 2;
    return !(*p == 11 && *(1 + a) == 5 && *(a + 2) == 7 && *(p - 2) == 5 && p - a == 3);
}"#,
        ),
        (
            "unevaluated_valid",
            "sizeof validates valid scalar operators without evaluating them",
            r#"
int main(void) {
    int n = 1;
    double d = 2.0;
    return !(sizeof(++n * 2) == sizeof(int) && sizeof(-d) == sizeof(double) &&
             sizeof(n && ++n) == sizeof(int) && n == 1);
}"#,
        ),
        (
            "reverse_subscript",
            "integer-first subscripting accepts signed and enum indices",
            r#"
enum E { INDEX = 2 };
int main(void) {
    int a[3] = {3, 5, 7};
    signed char n = -1;
    int *p = a + 1;
    return !(1 [a] == 5 && INDEX[a] == 7 && n[p] == 3);
}"#,
        ),
        (
            "reverse_subscript_effects",
            "subscript operand normalization evaluates each side exactly once",
            r#"
int a[3] = {3, 5, 7};
int calls;
int *get(void) {
    calls++;
    return a;
}
int main(void) {
    int n = 1;
    int value = n++ [get()];
    return !(value == 5 && n == 2 && calls == 1);
}"#,
        ),
        (
            "reverse_nested_subscript",
            "integer-first subscripting preserves row and record element types",
            r#"
struct S {
    int n;
};
int main(void) {
    int a[2][2] = {{3, 5}, {7, 11}};
    int (*p)[2] = a;
    struct S s[2] = {{13}, {17}};
    return !(1 [p][0] == 7 && 1 [s].n == 17);
}"#,
        ),
        (
            "additive_arithmetic",
            "numeric addition and subtraction accept mixed arithmetic operands",
            r#"
int main(void) {
    float f = 1.5f;
    double d = 4.0;
    long double l = 3.5L;
    return !(f + 2 == 3.5f && d - f == 2.5 && l + f == 5.0L && l - 2 == 1.5L);
}"#,
        ),
        (
            "compound_pointer_layouts",
            "compound pointer steps scale complete record and array elements",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S s[3] = {{3}, {5}, {7}};
    int a[3][2] = {{1, 2}, {3, 4}, {5, 6}};
    struct S *p = s;
    int (*q)[2] = a;
    p += 2;
    q += 2;
    p -= 1;
    q -= 1;
    return !(p->n == 5 && (*q)[1] == 4);
}"#,
        ),
        (
            "reverse_subscript_lvalue",
            "reversed indexing preserves const reads and modifiable element lvalues",
            r#"
int main(void) {
    const int c[2] = {5, 7};
    int a[2] = {1, 3};
    1 [a] = 9;
    0 [a] += 4;
    return !(1 [c] == 7 && a[0] == 5 && a[1] == 9);
}"#,
        ),
        (
            "nested_void_conditional_discarded",
            "nested void conditionals retain their selected side effects when discarded",
            r#"
int calls;
void tick(void) { calls++; }
int main(void) {
    int flag = 1;
    flag ? (flag ? tick() : (void)0) : (void)0;
    flag = 0;
    flag ? (void)0 : (flag ? (void)0 : tick());
    return calls != 2;
}"#,
        ),
    ] {
        cases.push(Case::new(
            format!("operand_{}", id),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
