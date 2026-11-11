// Invalid declarations cases for storage.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Invalid declarations distinguish binding errors from type/value constraints.
    for (purpose, source, diagnostic) in [
        (
            "local static cannot load automatic initializer",
            "int main(void){int n=7;static int x=n;return x;}",
            "E008",
        ),
        (
            "local static cannot load parameter initializer",
            "int f(int n){static int x=n;return x;}int main(void){return f(3);}",
            "E008",
        ),
        (
            "local static cannot call initializer",
            "int f(void){return 7;}int main(void){static int x=f();return x;}",
            "E008",
        ),
        (
            "local static cannot take automatic object address",
            "int main(void){int x=7;static int *p=&x;return *p;}",
            "E008",
        ),
        (
            "local static cannot take parameter address",
            "int f(int n){static int *p=&n;return *p;}int main(void){return f(3);}",
            "E008",
        ),
        (
            "local static cannot load const object initializer",
            "int main(void){const int n=7;static int x=n;return x;}",
            "E008",
        ),
        (
            "local static cannot copy static pointer value",
            "int main(void){static int x=7;static int *p=&x;static int *q=p;return *q;}",
            "E008",
        ),
        (
            "local static aggregate cannot load runtime element",
            "int main(void){int n=7;static int a[2]={3,n};return a[1];}",
            "E008",
        ),
        (
            "local static incomplete array needs initializer",
            "int main(void){static int a[];return 0;}",
            "E008",
        ),
        (
            "local static incomplete record rejected",
            "struct S;int main(void){static struct S x;return 0;}",
            "E008",
        ),
        (
            "local static void object rejected",
            "int main(void){static void x;return 0;}",
            "E008",
        ),
        (
            "local static same-scope redeclaration",
            "int main(void){static int x;static int x;return x;}",
            "E009",
        ),
        (
            "local static conflicts with automatic in same scope",
            "int main(void){int x;static int x;return 0;}",
            "E009",
        ),
        (
            "automatic conflicts with local static in same scope",
            "int main(void){static int x;int x;return 0;}",
            "E009",
        ),
        (
            "local static conflicts with parameter",
            "int f(int x){static int x;return x;}int main(void){return f(3);}",
            "E009",
        ),
        (
            "local static name does not escape block",
            "int main(void){{static int x=7;}return x;}",
            "E009",
        ),
        (
            "local static name does not escape function",
            "int f(void){static int x=7;return x;}int main(void){return x;}",
            "E009",
        ),
        (
            "block extern initializer is invalid",
            "int main(void){extern int x=7;return x;}",
            "E008",
        ),
        (
            "block extern name does not escape block",
            "int main(void){{extern int x;}return x;}int x=7;",
            "E009",
        ),
        (
            "block extern name does not escape function",
            "int f(void){extern int x;return x;}int main(void){return x;}int x=7;",
            "E009",
        ),
        (
            "block function name does not escape block",
            "int main(void){{int f(void);}return &f!=0;}int f(void){return 7;}",
            "E009",
        ),
        (
            "block function name does not escape function",
            "int g(void){int f(void);return f();}int main(void){return &f!=0;}int f(void){return 7;}",
            "E009",
        ),
        (
            "extern conflicts with same-scope automatic",
            "int main(void){int x;extern int x;return x;}int x;",
            "E009",
        ),
        (
            "automatic conflicts with same-scope extern",
            "int main(void){extern int x;int x;return x;}int x;",
            "E009",
        ),
        (
            "extern conflicts with same-scope static",
            "int main(void){static int x;extern int x;return x;}",
            "E009",
        ),
        (
            "extern conflicts with parameter",
            "int f(int x){extern int x;return x;}int x;int main(void){return f(3);}",
            "E009",
        ),
        (
            "extern scalar type conflicts with visible global",
            "int x;int main(void){extern long x;return 0;}",
            "E008",
        ),
        (
            "extern scalar type conflicts with later global",
            "int f(void){extern long x;return 0;}int x;int main(void){return f();}",
            "E008",
        ),
        (
            "extern scalar declarations disagree in block",
            "int main(void){extern int x;extern long x;return 0;}",
            "E008",
        ),
        (
            "extern scalar declarations disagree across blocks",
            "int main(void){{extern int x;}{extern long x;}return 0;}",
            "E008",
        ),
        (
            "extern array bounds conflict",
            "int a[3];int main(void){extern int a[4];return 0;}",
            "E008",
        ),
        (
            "extern array element types conflict",
            "int a[3];int main(void){extern long a[3];return 0;}",
            "E008",
        ),
        (
            "inner array bound does not complete outer block alias",
            "int f(void){\
                extern int a[];\
                {\
                    extern int a[3];\
                }\
                return sizeof(a);\
            }\
            int a[3];\
            int main(void){\
                return 0;\
            }",
            "E008",
        ),
        (
            "block array bound does not complete file scope declaration",
            "extern int a[];\
            int f(void){\
                extern int a[3];\
                return sizeof(a);\
            }\
            int main(void){\
                return sizeof(a);\
            }\
            int a[3];",
            "E008",
        ),
        (
            "other function array bound does not complete block alias",
            "int f(void){\
                extern int a[3];\
                return sizeof(a);\
            }\
            int g(void){\
                extern int a[];\
                return sizeof(a);\
            }\
            int a[3];\
            int main(void){\
                return 0;\
            }",
            "E008",
        ),
        (
            "tentative one-element array conflicts with block bound",
            "int a[];int f(void){extern int a[3];return sizeof(a);}int main(void){return f();}",
            "E008",
        ),
        (
            "hidden extern bound constrains later inferred definition",
            "int f(void){extern int a[3];return sizeof(a);}int a[]={1,2};int main(void){return f();}",
            "E008",
        ),
        (
            "block function return type conflicts",
            "int f(void);int main(void){long f(void);return 0;}",
            "E008",
        ),
        (
            "block function parameter type conflicts",
            "int f(int);int main(void){int f(long);return 0;}",
            "E008",
        ),
        (
            "block function conflicts with later definition",
            "int main(void){int f(long);return 0;}int f(int n){return n;}",
            "E008",
        ),
        (
            "block object conflicts with visible function",
            "int f(void);int main(void){extern int f;return 0;}",
            "E008",
        ),
        (
            "block function conflicts with visible object",
            "int f;int main(void){int f(void);return 0;}",
            "E008",
        ),
        (
            "block object conflicts with later function",
            "int main(void){extern int f;return 0;}int f(void){return 7;}",
            "E008",
        ),
        (
            "block function conflicts with later object",
            "int main(void){int f(void);return 0;}int f;",
            "E008",
        ),
        (
            "block function cannot have static storage class",
            "int main(void){static int f(void);return 0;}",
            "E008",
        ),
        (
            "const local static remains nonmodifiable",
            "int main(void){static const int x=7;x=9;return x;}",
            "E008",
        ),
    ] {
        cases.push(Case::new(
            format!("storage_{:03}", cases.len()+1),
            purpose,
            source,
            Expectation::Reject(diagnostic.into()),
        ));
    }
}
