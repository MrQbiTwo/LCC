// Objects cases for address initializer.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "scalar address",
            "int x=37;int *p=&x;int main(void){return *p;}",
            37,
        ),
        (
            "write through initializer",
            "int x=3;int *p=&x;int main(void){*p=19;return x;}",
            19,
        ),
        (
            "zero-initialized target",
            "int x;int *p=&x;int main(void){*p=13;return x;}",
            13,
        ),
        (
            "forward external object",
            "extern int x;int *p=&x;int x=23;int main(void){return *p;}",
            23,
        ),
        (
            "static object",
            "static int x=17;static int *p=&x;int main(void){return *p;}",
            17,
        ),
        (
            "qualified object",
            "const volatile int x=21;const volatile int *p=&x;int main(void){return *p;}",
            21,
        ),
        (
            "array decay",
            "int a[]={3,5,7};int *p=a;int main(void){return p[2];}",
            7,
        ),
        (
            "positive element addend",
            "int a[]={3,5,7};int *p=a+2;int main(void){return *p;}",
            7,
        ),
        (
            "integer plus address",
            "int a[]={3,5,7};int *p=1+a;int main(void){return *p;}",
            5,
        ),
        (
            "negative element addend",
            "int a[]={3,5,7};int *p=&a[2]-1;int main(void){return *p;}",
            5,
        ),
        (
            "one-past address",
            "int a[3];int *p=a+3;int main(void){return p-a;}",
            3,
        ),
        (
            "folded integer displacement",
            "int a[]={1,2,3,4};int *p=a+(7-1)/2;int main(void){return *p;}",
            4,
        ),
        (
            "enum displacement",
            "enum E{N=2};int a[]={3,5,7};int *p=a+N;int main(void){return *p;}",
            7,
        ),
        (
            "negative folded displacement",
            "int a[]={3,5,7};int *p=&a[2]+(-2);int main(void){return *p;}",
            3,
        ),
        (
            "short element scaling",
            "short a[]={3,5,11};short *p=a+2;int main(void){return *p;}",
            11,
        ),
        (
            "double element scaling",
            "double a[]={1.5,2.5,4.5};double *p=a+2;int main(void){return (int)*p;}",
            4,
        ),
        (
            "binary80 element scaling",
            "long double a[]={1.25L,2.5L,7.75L};long double *p=&a[2]-1;int main(void){return (int)*p;}",
            2,
        ),
        (
            "row pointer scaling",
            "int a[3][2]={{1,2},{3,4},{5,6}};int (*p)[2]=a+1;int main(void){return p[1][1];}",
            6,
        ),
        (
            "whole-array address",
            "int a[]={3,5,7};int (*p)[3]=&a;int main(void){return (*p)[2]+sizeof(*p);}",
            19,
        ),
        (
            "nested subscript address",
            "int a[2][3]={{1,2,3},{4,5,6}};int *p=&a[1][2];int main(void){return *p;}",
            6,
        ),
        (
            "nested array decay",
            "int a[2][3]={{1,2,3},{4,5,6}};int *p=a[1]+1;int main(void){return *p;}",
            5,
        ),
        (
            "pointer object address",
            "int x=9;int *p=&x;int **q=&p;int main(void){return **q;}",
            9,
        ),
        (
            "pointer table element address",
            "int x=9,y=13;int *a[]={&x,&y};int **p=&a[1];int main(void){return **p;}",
            13,
        ),
        (
            "self address",
            "void *p=&p;int main(void){return p==&p;}",
            1,
        ),
        (
            "cyclic record references",
            "struct N{\
                int value;\
                struct N *next;\
            };\
            extern struct N b;\
            struct N a={\
                3,&b};\
            struct N b={\
                4,&a};\
            int main(void){\
                return a.next->value+b.next->value;\
            }",
            7,
        ),
        (
            "padded member address",
            "struct S{char c;long x;};struct S s={1,29};long *p=&s.x;int main(void){return *p;}",
            29,
        ),
        (
            "nested record member",
            "struct I{\
                char c;\
                int x;\
            };\
            struct O{\
                double d;\
                struct I i;\
            };\
            struct O s={\
                2.0,{\
                    1,31}};\
                int *p=&s.i.x;\
                int main(void){\
                    return *p;\
                }",
            31,
        ),
        (
            "array member decay",
            "struct S{char c;int a[3];};struct S s={1,{3,5,7}};int *p=s.a+2;int main(void){return *p;}",
            7,
        ),
        (
            "member through constant arrow",
            "struct S{int x,y;};struct S s={7,11};int *p=&(&s)->y;int main(void){return *p;}",
            11,
        ),
        (
            "union member address",
            "union U{long x;char a[8];};union U u={41};long *p=&u.x;int main(void){return *p;}",
            41,
        ),
        (
            "interleaved pointers and bit-fields",
            "int x=5,y=7;\
            struct S{\
                char c;\
                int *p;\
                unsigned a:3;\
                signed int b:5;\
                int *q;\
                char z;\
            };\
            struct S s={\
                1,&x,6,-3,&y,2};\
            int main(void){\
                return s.c+*s.p+s.a+s.b+*s.q+s.z;\
            }",
            18,
        ),
        (
            "pointer aggregate array",
            "int a[]={\
                3,5,7};\
            struct S{\
                int *p;\
                char c;\
            };\
            struct S s[]={{\
                a+2,1},{\
                a,2}};\
            int main(void){\
                return *s[0].p+*s[1].p+s[0].c+s[1].c;\
            }",
            13,
        ),
] {
        cases.push(Case::new(
            format!("address_initializer_{:03}", cases.len()+1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
}
