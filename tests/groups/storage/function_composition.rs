// Function composition cases for storage.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Declaration composition must retain definition arity and ignore only
    // parameter-object qualifiers, while preserving the actual body homes.
    let mut constraint_index = 0;
    for (purpose, source, expected) in [
        (
            "void prototype composes with empty-list definition",
            "int f(void);int f(){return 7;}int main(void){return f();}",
            7,
        ),
        (
            "empty-list definition composes with later void prototype",
            "int f(){return 7;}int f(void);int main(void){return f();}",
            7,
        ),
        (
            "hidden void prototype composes with empty-list definition",
            "int g(void){int f(void);return f();}int f(){return 7;}int main(void){return g();}",
            7,
        ),
        (
            "empty-list definition composes with block void prototype",
            "int f(){return 7;}int main(void){int f(void);return f();}",
            7,
        ),
        (
            "const scalar prototype permits unqualified definition parameter",
            "int f(const int);int f(int n){return n+1;}int main(void){return f(7);}",
            8,
        ),
        (
            "volatile scalar prototype permits unqualified definition parameter",
            "int f(volatile int);int f(int n){return n+2;}int main(void){return f(7);}",
            9,
        ),
        (
            "scalar definition retains outer const qualification",
            "int f(int);int f(const int n){return n+3;}int f(int);int main(void){return f(7);}",
            10,
        ),
        (
            "scalar definition retains outer volatile qualification",
            "int f(int);int f(volatile int n){n++;return n;}int f(int);int main(void){return f(7);}",
            8,
        ),
        (
            "const pointer prototype permits unqualified definition parameter",
            "int f(int *const);int f(int *p){return ++*p;}int main(void){int n=7;return f(&n);}",
            8,
        ),
        (
            "volatile pointer prototype permits unqualified definition parameter",
            "int f(int *volatile);int f(int *p){return *p+2;}int main(void){int n=7;return f(&n);}",
            9,
        ),
        (
            "pointer definition retains outer const qualification",
            "int f(int *);\
            int f(int *const p){\
                return *p+3;\
            }\
            int f(int *);\
            int main(void){\
                int n=7;\
                return f(&n);\
            }",
            10,
        ),
        (
            "pointer definition retains outer volatile qualification",
            "int f(int *);\
            int f(int *volatile p){\
                return *p+4;\
            }\
            int f(int *);\
            int main(void){\
                int n=7;\
                return f(&n);\
            }",
            11,
        ),
        (
            "block prototype ignores scalar outer qualifiers",
            "int f(int n){return n+1;}int main(void){int f(const volatile int);return f(7);}",
            8,
        ),
        (
            "block prototype ignores pointer outer qualifiers",
            "int f(int *p){\
                return *p+1;\
            }\
            int main(void){\
                int n=7;\
                int f(int *const volatile);\
                return f(&n);\
            }",
            8,
        ),
        (
            "unspecified declaration composes with int prototype",
            "int f();int f(int n){return n+1;}int main(void){return f(7);}",
            8,
        ),
        (
            "int definition composes with later unspecified declaration",
            "int f(int n){return n+2;}int f();int main(void){return f(7);}",
            9,
        ),
        (
            "unspecified declaration composes with double prototype",
            "int f();int f(double n){return (int)(n+1.0);}int main(void){return f(7.0f);}",
            8,
        ),
        (
            "double definition composes with later unspecified declaration",
            "int f(double n){return (int)(n+2.0);}int f();int main(void){return f(7.0f);}",
            9,
        ),
        (
            "unspecified declaration permits qualified promoted parameter",
            "int f();int f(const volatile int n){return n+3;}int main(void){return f(7);}",
            10,
        ),
        (
            "register definition parameter has an ordinary argument home",
            "int f(register int n){n++;return n;}int main(void){return f(7);}",
            8,
        ),
        (
            "register prototype parameter composes with ordinary definition",
            "int f(register int);int f(int n){return n+2;}int main(void){return f(7);}",
            9,
        ),
        (
            "file extern incomplete record can remain unused",
            "struct S;extern struct S missing;int main(void){return 7;}",
            7,
        ),
        (
            "block extern incomplete record can remain unused",
            "struct S;int main(void){extern struct S missing;return 7;}",
            7,
        ),
        (
            "extern record declaration observes later tag completion",
            "struct S;extern struct S x;struct S{int n;};struct S x={7};int main(void){return x.n;}",
            7,
        ),
        (
            "block extern incomplete record can supply an address",
            "struct S;\
            int f(void){\
                extern struct S x;\
                return &x!=0;\
            }\
            struct S{\
                int n;\
            };\
            struct S x={7};\
            int main(void){\
                return f();\
            }",
            1,
        ),
    ] {
        constraint_index += 1;
        cases.push(Case::new(
            format!("storage_constraint_{constraint_index:03}"),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
    // E008 covers incompatible types, invalid storage classes and argument
    // counts; these declarations do not introduce duplicate lexical objects.
    for (purpose, source) in [
        (
            "nonempty prototype cannot precede empty-list definition",
            "int f(int);int f(){return 7;}int main(void){return f(3);}",
        ),
        (
            "nonempty prototype cannot follow empty-list definition",
            "int f(){return 7;}int f(int);int main(void){return f(3);}",
        ),
        (
            "hidden nonempty prototype cannot precede empty-list definition",
            "int g(void){int f(int);return f(3);}int f(){return 7;}int main(void){return g();}",
        ),
        (
            "block nonempty prototype cannot follow empty-list definition",
            "int f(){return 7;}int main(void){int f(int);return f(3);}",
        ),
        (
            "void prototype argument checks survive empty-list definition",
            "int f(void);int f(){return 7;}int main(void){return f(3);}",
        ),
        (
            "later void prototype checks calls after empty-list definition",
            "int f(){return 7;}int f(void);int main(void){return f(3);}",
        ),
        (
            "int prototype argument checks survive unspecified redeclaration",
            "int f(int n){return n;}int f();int main(void){return f();}",
        ),
        (
            "pointer pointee const qualification remains significant",
            "int f(const int *);int f(int *p){return *p;}int main(void){return 0;}",
        ),
        (
            "pointer pointee volatile qualification remains significant",
            "int f(volatile int *);int f(int *p){return *p;}int main(void){return 0;}",
        ),
        (
            "block prototype cannot discard pointee qualification",
            "int f(const int *p){return *p;}int main(void){int f(int *);return 0;}",
        ),
        (
            "outer pointer qualifiers do not hide pointee mismatch",
            "int f(const int *const);int f(int *volatile p){return *p;}int main(void){return 0;}",
        ),
        (
            "qualified definition parameter remains nonmodifiable",
            "int f(int);int f(const int n){n++;return n;}int main(void){return f(7);}",
        ),
        (
            "unspecified declaration rejects char prototype",
            "int f();int f(char);int main(void){return 0;}",
        ),
        (
            "char prototype rejects later unspecified declaration",
            "int f(char);int f();int main(void){return 0;}",
        ),
        (
            "unspecified declaration rejects short prototype",
            "int f();int f(short);int main(void){return 0;}",
        ),
        (
            "unspecified declaration rejects float prototype",
            "int f();int f(float);int main(void){return 0;}",
        ),
        (
            "float prototype rejects later unspecified declaration",
            "int f(float);int f();int main(void){return 0;}",
        ),
        (
            "unspecified declaration rejects variadic prototype",
            "int f();int f(int,...);int main(void){return 0;}",
        ),
        (
            "variadic prototype rejects later unspecified declaration",
            "int f(int,...);int f();int main(void){return 0;}",
        ),
        (
            "block unspecified declaration rejects nonpromoted prototype",
            "int f(char);int main(void){int f();return 0;}",
        ),
        (
            "file object cannot combine static and extern",
            "static extern int x;int main(void){return 0;}",
        ),
        (
            "block object cannot combine auto and register",
            "int main(void){auto register int x;return 0;}",
        ),
        (
            "typedef cannot combine with extern",
            "typedef extern int T;int main(void){return 0;}",
        ),
        (
            "file object cannot repeat static storage class",
            "static static int x;int main(void){return 0;}",
        ),
        (
            "block extern cannot repeat its storage class",
            "int main(void){extern extern int x;return 0;}",
        ),
        (
            "typedef cannot repeat its storage class",
            "typedef typedef int T;int main(void){return 0;}",
        ),
        (
            "block register cannot repeat its storage class",
            "int main(void){register register int x;return 0;}",
        ),
        (
            "file object cannot use auto storage class",
            "auto int x;int main(void){return 0;}",
        ),
        (
            "file object cannot use register storage class",
            "register int x;int main(void){return 0;}",
        ),
        (
            "block function cannot use auto storage class",
            "int main(void){auto int f(void);return 0;}",
        ),
        (
            "block function cannot use register storage class",
            "int main(void){register int f(void);return 0;}",
        ),
        (
            "function parameter cannot use static storage class",
            "int f(static int n){return n;}int main(void){return 0;}",
        ),
        (
            "function parameter cannot use extern storage class",
            "int f(extern int n){return n;}int main(void){return 0;}",
        ),
        (
            "function parameter cannot use typedef storage class",
            "int f(typedef int n){return n;}int main(void){return 0;}",
        ),
        (
            "function parameter cannot use auto storage class",
            "int f(auto int n){return n;}int main(void){return 0;}",
        ),
        (
            "incomplete extern record still rejects sizeof",
            "struct S;extern struct S x;int main(void){return sizeof(x);}",
        ),
    ] {
        constraint_index += 1;
        cases.push(Case::new(
            format!("storage_constraint_{constraint_index:03}"),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
