// Constraints cases for array bounds.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, code) in [
        (
            "uninitialized automatic array",
            "int main(void){int a[];return 0;}",
            "E008",
        ),
        (
            "typedef stays incomplete",
            "typedef int A[];A a={1,2};int main(void){return sizeof(A);}",
            "E008",
        ),
        (
            "local typedef stays incomplete",
            "int main(void){typedef int A[];A a={1,2};return sizeof(A);}",
            "E008",
        ),
        (
            "sizeof pending initializer",
            "int main(void){int a[]={sizeof(a)};return 0;}",
            "E008",
        ),
        (
            "sizeof pending file object",
            "int a[];int main(void){return sizeof(a);}",
            "E008",
        ),
        (
            "unknown inner bound",
            "int a[2][]={{1},{2}};int main(void){return 0;}",
            "E008",
        ),
        (
            "multiple unknown bounds",
            "int a[][]={{1}};int main(void){return 0;}",
            "E008",
        ),
        (
            "incomplete record element",
            "struct S;struct S a[];int main(void){return 0;}",
            "E008",
        ),
        (
            "void element",
            "void a[]={0};int main(void){return 0;}",
            "E008",
        ),
        (
            "explicit void element",
            "void a[2];int main(void){return 0;}",
            "E008",
        ),
        (
            "function element",
            "int a[]();int main(void){return 0;}",
            "E008",
        ),
        (
            "unknown member bound",
            "struct S{int a[];};int main(void){return 0;}",
            "E008",
        ),
        (
            "internal tentative incomplete",
            "static int a[];int main(void){return 0;}",
            "E008",
        ),
        (
            "conflicting complete bounds",
            "int a[2];int a[3];int main(void){return 0;}",
            "E008",
        ),
        (
            "conflicting element kinds",
            "extern int a[];double a[]={1.0};int main(void){return 0;}",
            "E008",
        ),
        (
            "conflicting element qualifiers",
            "extern const int a[];int a[]={1};int main(void){return 0;}",
            "E008",
        ),
        (
            "conflicting inner bounds",
            "extern int a[][2];int a[][3]={{1}};int main(void){return 0;}",
            "E008",
        ),
        (
            "record identity mismatch",
            "struct S{\
                int x;\
            };\
            struct T{\
                int x;\
            };\
            extern struct S a[];\
            struct T a[]={{\
                1}};\
            int main(void){\
                return 0;\
            }",
            "E008",
        ),
        (
            "scalar versus array",
            "int a;int a[]={1};int main(void){return 0;}",
            "E008",
        ),
        (
            "array versus scalar",
            "int a[]={1};int a;int main(void){return 0;}",
            "E008",
        ),
        (
            "const inferred element assignment",
            "int main(void){const int a[]={1};a[0]=2;return 0;}",
            "E008",
        ),
        (
            "empty initializer list",
            "int a[]={};int main(void){return 0;}",
            "E006",
        ),
        (
            "missing brace initializer",
            "int a[]=1;int main(void){return 0;}",
            "E008",
        ),
        (
            "bound inherited from declaration",
            "extern int a[1];int a[]={1,2};int main(void){return 0;}",
            "E006",
        ),
        (
            "non-character string",
            "int a[]=\"abc\";int main(void){return 0;}",
            "E008",
        ),
        (
            "incomplete pointer arithmetic",
            "int main(void){int (*p)[];p++;return 0;}",
            "E008",
        ),
        (
            "inferred byte-size overflow",
            "int a[][268435456]={{1},{2}};int main(void){return 0;}",
            "E008",
        ),
    ] {
        cases.push(Case::new(
            format!("array_bounds_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::Reject(code.into()),
        ));
    }
}
