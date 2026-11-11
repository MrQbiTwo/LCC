// Scalar controls cases for operand constraints.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "integer_controls",
            "integer controlling expressions retain all statement forms",
            r#"
int main(void) {
    int n = 0;
    int flag = 3;
    if (flag)
        n++;
    while (n < 3)
        n++;
    do {
        n++;
    } while (n < 5);
    for (; n < 7; n++)
        ;
    return n == 7 ? 0 : 1;
}"#,
        ),
        (
            "floating_controls",
            "float, double, and binary80 values are scalar controls",
            r#"
int main(void) {
    float f = 0.5f;
    double d = 2.0;
    long double l = 3.0L;
    int n = 0;
    if (f)
        n++;
    while (d) {
        n++;
        d = 0.0;
    }
    do {
        n++;
    } while (!l);
    for (; l; l = 0.0L)
        n++;
    return !(n == 4 && f && d == 0.0 && !l);
}"#,
        ),
        (
            "enum_bitfield_controls",
            "enumerations and bit fields retain scalar truth conversion",
            r#"
enum E { ZERO, ONE };
struct S {
    unsigned n : 3;
    signed int m : 4;
};
int main(void) {
    enum E e = ONE;
    struct S s = {3, -2};
    return !(e && s.n && s.m && !!s.n && (s.m ? 1 : 0));
}"#,
        ),
        (
            "incomplete_pointer_controls",
            "pointer truth does not require a complete pointed-to record",
            r#"
struct S;
int check(struct S *p) {
    if (p)
        return !p;
    while (p)
        return 1;
    do {
    } while (p);
    for (; p;)
        return 1;
    return p ? 1 : 0;
}
int main(void) { return check((struct S *)0); }"#,
        ),
        (
            "function_designator_controls",
            "function designators decay for logical and control expressions",
            r#"
int f(void) { return 3; }
int main(void) {
    int n = 0;
    if (f)
        n++;
    while (f) {
        n++;
        break;
    }
    do {
        n++;
    } while (!f);
    for (; f;) {
        n++;
        break;
    }
    return !(n == 4 && f && !!f && (f ? f() : 0) == 3);
}"#,
        ),
        (
            "array_designator_controls",
            "array designators decay for scalar controls and logical operations",
            r#"
int main(void) {
    int a[2] = {3, 5};
    int n = 0;
    if (a)
        n++;
    while (a) {
        n++;
        break;
    }
    do {
        n++;
    } while (!a);
    for (; a;) {
        n++;
        break;
    }
    return !(n == 4 && a && !!a && (a ? a[1] : 0) == 5);
}"#,
        ),
        (
            "logical_short_circuit",
            "logical scalar validation preserves short-circuit side effects",
            r#"
int main(void) {
    int n = 0;
    int x = 0;
    int y = 0;
    x = 0 && ++n;
    y = 1 || ++n;
    x = 1 && ++n;
    y = 0 || ++n;
    return !(n == 2 && x == 1 && y == 1);
}"#,
        ),
        (
            "record_conditional_arms",
            "only the condition of a record conditional must be scalar",
            r#"
struct S {
    int n;
};
int main(void) {
    struct S a = {3}, b = {5}, c;
    int flag = 0;
    c = flag ? a : b;
    return c.n != 5;
}"#,
        ),
        (
            "void_conditional_arms",
            "conditional arms may both have void type",
            r#"
int n;
void add(void) { n++; }
int main(void) {
    int flag = 1;
    flag ? add() : (void)0;
    flag = 0;
    flag ? (void)0 : add();
    return n != 2;
}"#,
        ),
        (
            "discarded_loop_clauses",
            "for initialization and iteration may discard record or void values",
            r#"
struct S {
    int n;
};
void noop(void) {}
int main(void) {
    struct S s = {3};
    int n = 0;
    for (s; n < 2; noop())
        n++;
    for (noop(); n < 4; s)
        n++;
    return n != 4;
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
