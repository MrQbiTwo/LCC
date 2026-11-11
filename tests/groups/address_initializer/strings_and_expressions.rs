// Strings and expressions cases for address initializer.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "string pointer",
            "char *p=\"abc\";int main(void){return p[0]+p[3];}",
            97,
        ),
        (
            "empty string pointer",
            "char *p=\"\";int main(void){return *p;}",
            0,
        ),
        (
            "embedded null and concatenation",
            "char *p=\"a\\0\" \"bc\";int main(void){return p[1]+p[3]+p[4];}",
            99,
        ),
        (
            "string addend",
            "char *p=\"abcd\"+2;int main(void){return *p;}",
            99,
        ),
        (
            "string element address",
            "char *p=&\"abcd\"[3]-2;int main(void){return *p;}",
            98,
        ),
        (
            "string pointer table",
            "char *a[]={\"ab\",\"cd\"+1,\"\"};int main(void){return a[0][1]+a[1][0]+a[2][0];}",
            198,
        ),
        (
            "function designator",
            "int f(int x){return x+7;}int (*p)(int)=f;int main(void){return p(3);}",
            10,
        ),
        (
            "function address",
            "int f(int x){return x+7;}int (*p)(int)=&f;int main(void){return p(4);}",
            11,
        ),
        (
            "forward function designator",
            "int f(int);int (*p)(int)=f;int f(int x){return x+9;}int main(void){return p(4);}",
            13,
        ),
        (
            "function pointer table",
            "int f(int x){\
                return x+3;\
            }\
            int g(int x){\
                return x*2;\
            }\
            int (*a[])(int)={\
                f,&g};\
            int main(void){\
                return a[0](4)+a[1](5);\
            }",
            17,
        ),
        (
            "pointer cast changes scaling",
            "int a[]={3,5,7};int *p=(int *)((char *)a+8);int main(void){return *p;}",
            7,
        ),
        (
            "signed integer address round trip",
            "int x=33;long n=(long)&x;int main(void){return *(int *)n;}",
            33,
        ),
        (
            "unsigned integer address addend",
            "int a[]={3,5,7};unsigned long n=(unsigned long)a+8;int main(void){return *(int *)n;}",
            7,
        ),
        (
            "integer to pointer cast preserves symbol",
            "int x=35;int *p=(int *)(unsigned long)&x;int main(void){return *p;}",
            35,
        ),
        (
            "null pointer literals",
            "int *p=0;void *q=(void *)0;int main(void){return p==0&&q==0;}",
            1,
        ),
        (
            "numeric pointer representation",
            "void *p=(void *)305419896UL;int main(void){return (unsigned long)p==305419896UL;}",
            1,
        ),
        (
            "selected symbolic conditional",
            "int x=7,y=11;int *p=(2<3)?&x:&y;int main(void){return *p;}",
            7,
        ),
        (
            "selected null conditional",
            "int x=7;int *p=0?&x:0;int main(void){return p==0;}",
            1,
        ),
        (
            "braced pointer scalar",
            "int x=43;int *p={&x};int main(void){return *p;}",
            43,
        ),
        (
            "conditional array scaling",
            "int a[]={1,2},b[]={3,4};int *p=(1?a:b)+1;int main(void){return *p;}",
            2,
        ),
        (
            "conditional false array scaling",
            "int a[]={1,2},b[]={3,4};int *p=(0?a:b)+1;int main(void){return *p;}",
            4,
        ),
        (
            "conditional integer null arm",
            "int a[]={3,7};int *p=(1?a:0)+1;int main(void){return *p;}",
            7,
        ),
        (
            "conditional void null arm",
            "int a[]={3,7};int *p=(1?a:(void*)0)+1;int main(void){return *p;}",
            7,
        ),
        (
            "conditional folded void null arm",
            "int a[]={3,7};int *p=(0?(void*)(1-1):a)+1;int main(void){return *p;}",
            7,
        ),
        (
            "conditional pointee qualifiers",
            "int a[]={3,7};const int b[]={4,8};const int *p=(1?a:b)+1;int main(void){return *p;}",
            7,
        ),
        (
            "runtime conditional scaling",
            "int main(void){int a[]={1,2},b[]={3,4};int n=0;return *((n?a:b)+1);}",
            4,
        ),
        (
            "conditional function designators",
            "int f(int x){\
                return x+3;\
            }\
            int g(int x){\
                return x+7;\
            }\
            int (*p)(int)=1?f:g;\
            int main(void){\
                int n=0;\
                return p(1)+(n?f:g)(2);\
            }",
            13,
        ),
        (
            "conditional function and void null",
            "int f(int x){return x+3;}int (*p)(int)=1?f:(void*)0;int main(void){return p(4);}",
            7,
        ),
        (
            "runtime void pointer arm",
            "int x=9;int main(void){int n=0;void *v=&x;int *p=&x;return *(int *)(n?p:v);}",
            9,
        ),
        (
            "numeric pointer scaling",
            "int *p=(int*)16+3;int main(void){return (unsigned long)p;}",
            28,
        ),
        (
            "numeric pointer subtraction",
            "int *p=(int*)32-3;int main(void){return (unsigned long)p;}",
            20,
        ),
        (
            "numeric pointer nested integer addition",
            "unsigned long n=(unsigned long)((int*)16+1)+4;int main(void){return n;}",
            24,
        ),
        (
            "numeric pointer nested integer multiplication",
            "unsigned long n=(unsigned long)((int*)16+1)*2;int main(void){return n;}",
            40,
        ),
        (
            "numeric pointer nested floating conversion",
            "double n=(double)(unsigned long)((int*)16+1)+0.5;int main(void){return n==20.5;}",
            1,
        ),
        (
            "numeric pointer conditional test",
            "int x=7,y=11;int *p=((unsigned long)((int*)16+1)==20)?&x:&y;int main(void){return *p;}",
            7,
        ),
        (
            "whole string array address",
            "char (*p)[4]=&\"abc\";int main(void){char (*q)[4]=&\"xyz\";return (*p)[1]+(*q)[3];}",
            98,
        ),
        (
            "address dereference cancellation",
            "int x=39;int *p=&*(&x);int main(void){return *p;}",
            39,
        ),
        (
            "negative integer relocation addend",
            "int a[3];unsigned long n=(unsigned long)a-4;int main(void){return n+4==(unsigned long)a;}",
            1,
        ),
        (
            "constant short circuit avoids runtime branch",
            "int f(void){\
                return 1;\
            }\
            int n=0&&f();\
            int x=7;\
            int *p=(1||f())?&x:0;\
            int main(void){\
                return n+*p;\
            }",
            7,
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
