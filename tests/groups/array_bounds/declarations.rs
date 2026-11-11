// Declarations cases for array bounds.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "deferred home and self address",
            "int main(void){int n=9;int a[]={(&a[1]-&a[0]),n};int b=7;return sizeof(a)+a[0]+a[1]+b;}",
            25,
        ),
        (
            "runtime expressions",
            "int n;\
            int f(void){\
                n++;\
                return n;\
            }\
            int main(void){\
                int a[]={\
                    f(),f(),f()};\
                return sizeof(a)+a[0]+a[1]+a[2]+n;\
            }",
            21,
        ),
        (
            "pointer elements",
            "int main(void){int x=3,y=4;int *a[]={&x,&y};return sizeof(a)+*a[0]+*a[1];}",
            23,
        ),
        (
            "aggregate call temporary before array home",
            "struct S{\
                long x,y,z;\
            };\
            struct S f(int x){\
                struct S s={\
                    x,2,3};\
                return s;\
            }\
            int main(void){\
                int n=9;\
                struct S a[]={\
                    f(1),f(4)};\
                int b=7;\
                return sizeof(a)+a[0].x+a[1].x+a[1].z+n+b;\
            }",
            72,
        ),
        (
            "incomplete parameter decay",
            "int f(int a[]){return sizeof(a)+a[1];}int main(void){int a[]={3,4};return f(a);}",
            12,
        ),
        (
            "multidimensional parameter decay",
            "int f(int a[][2]){return a[1][1];}int main(void){int a[][2]={{1,2},{3,4}};return f(a);}",
            4,
        ),
        (
            "extern then inferred definition",
            "extern int a[];\
            int f(void){\
                return a[1];\
            }\
            int a[]={\
                3,4};\
            int main(void){\
                return sizeof(a)+f();\
            }",
            12,
        ),
        (
            "tentative then inferred definition",
            "int a[];int a[]={3,4};int main(void){return sizeof(a)+a[1];}",
            12,
        ),
        (
            "explicit then unknown bound",
            "extern int a[4];int a[]={3,4};int main(void){return sizeof(a)+a[2]+a[3];}",
            16,
        ),
        (
            "unknown then explicit bound",
            "extern int a[];int a[4]={3,4};int main(void){return sizeof(a)+a[3];}",
            16,
        ),
        (
            "definition then extern",
            "int a[]={3,4};extern int a[];int main(void){return sizeof(a)+a[1];}",
            12,
        ),
        (
            "tentative completion at unit end",
            "int a[];int main(void){a[0]=7;return a[0];}",
            7,
        ),
        (
            "typedef tentative isolation",
            "typedef int A[];A a;A b={2,3};int main(void){return a[0]+sizeof(b)+b[1];}",
            11,
        ),
        (
            "nested compatible declarations",
            "extern int a[][2];int a[][2]={{1,2},{3,4}};int main(void){return sizeof(a)+a[1][1];}",
            20,
        ),
        (
            "pointer element redeclarations",
            "extern int *a[];int *a[2];int main(void){return sizeof(a);}",
            16,
        ),
        (
            "function pointer redeclarations",
            "extern int (*a[])(int);int (*a[2])(int);int main(void){return sizeof(a);}",
            16,
        ),
        (
            "static initialized array",
            "static int a[]={3,4};int main(void){return sizeof(a)+a[1];}",
            12,
        ),
        (
            "pointer to incomplete array",
            "extern int a[];int main(void){int (*p)[]=&a;return (*p)[1];}int a[]={3,4};",
            4,
        ),
    ] {
        cases.push(Case::new(
            format!("array_bounds_{:03}", cases.len()+1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
}
