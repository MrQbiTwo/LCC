// Foreign compilers independently read or supply static pointer relocations.
use crate::{Case, Expectation};

// Swap data producer and consumer to cover both directions of each boundary.
fn pair(cases: &mut Vec<Case>, name: &str, producer: &str, consumer: &str) {
    for (direction, source, foreign) in [("in", producer, consumer), ("out", consumer, producer)] {
        cases.push(Case::new(
            format!("interop_address_{name}_{direction}"),
            format!("foreign static address boundary {name} {direction}"),
            source,
            Expectation::CInterop(foreign.into()),
        ));
    }
}

// Check exact addends and object layout using GCC and Clang at both optimization levels.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (name, producer, consumer) in [
        (
            "object",
            "int x=37;int *p=&x;",
            "extern int x;extern int *p;int driver(void){int ok=p==&x&&*p==37;*p=19;return ok&&x==19;}",
        ),
        (
            "external_target",
            "extern int x;int *p=&x;",
            "int x=23;extern int *p;int driver(void){return p==&x&&*p==23;}",
        ),
        (
            "offsets",
            "int a[]={3,5,7,11};int *p=a+3;int *q=&a[3]-2;int *end=a+4;",
            "extern int a[];\
            extern int *p,*q,*end;\
            int driver(void){\
                return p==&a[3]&&q==&a[1]&&end==&a[4]&&*p==11&&*q==5;\
            }",
        ),
        (
            "rows",
            "int a[3][2]={{1,2},{3,4},{5,6}};int (*p)[2]=a+1;int (*whole)[3][2]=&a;int *q=&a[2][1];",
            "extern int a[3][2];\
            extern int (*p)[2];\
            extern int (*whole)[3][2];\
            extern int *q;\
            int driver(void){\
                return p==&a[1]&&whole==&a&&q==&a[2][1]&&p[1][1]==6&&(*whole)[1][0]==3;\
            }",
        ),
        (
            "pointer_table",
            "int x=3,y=7;int *a[]={&x,&y};int **p=a+1;",
            "extern int x,y;\
            extern int *a[];\
            extern int **p;\
            int driver(void){\
                return a[0]==&x&&a[1]==&y&&p==&a[1]&&**p==7;\
            }",
        ),
        (
            "strings",
            "char *p=\"a\\0bc\";char *q=\"wxyz\"+2;char *a[]={\"hi\",\"bye\"+1,\"\"};",
            "extern char *p,*q,*a[];\
            int driver(void){\
                return p[0]=='a'&&p[1]==0&&p[2]=='b'&&p[3]=='c'&&p[4]==0&&q[0]=='y'&&q[1]=='z'&&q[2]==0&&a[0][1]=='i'&&\
                a[1][0]=='y'&&a[2][0]==0;\
            }",
        ),
        (
            "function_table",
            "int f(int x){return x+3;}int g(int x){return x*2;}int (*a[])(int)={f,&g};",
            "int f(int);\
            int g(int);\
            extern int (*a[])(int);\
            int driver(void){\
                return a[0]==f&&a[1]==g&&a[0](4)==7&&a[1](5)==10;\
            }",
        ),
        (
            "external_function",
            "int target(int);int (*p)(int)=target;",
            "int target(int x){\
                return x+9;\
            }\
            extern int (*p)(int);\
            int driver(void){\
                return p==target&&p(4)==13;\
            }",
        ),
        (
            "record_padding",
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
                1,&x,6,-3,&y,2};",
            "struct S{\
                char c;\
                int *p;\
                unsigned a:3;\
                signed int b:5;\
                int *q;\
                char z;\
            };\
            extern int x,y;\
            extern struct S s;\
            int driver(void){\
                return sizeof(s)==40&&s.c==1&&s.p==&x&&*s.p==5&&s.a==6&&s.b==-3&&s.q==&y&&*s.q==7&&\
                s.z==2;\
            }",
        ),
        (
            "member_address",
            "struct I{\
                char c;\
                int a[3];\
            };\
            struct O{\
                double d;\
                struct I i;\
            };\
            struct O s={\
                1.5,{\
                    2,{\
                        3,5,7}}\
                };\
                int *p=&s.i.a[2];",
            "struct I{\
                char c;\
                int a[3];\
            };\
            struct O{\
                double d;\
                struct I i;\
            };\
            extern struct O s;\
            extern int *p;\
            int driver(void){\
                return p==&s.i.a[2]&&*p==7&&s.d==1.5&&s.i.c==2;\
            }",
        ),
        (
            "cyclic_records",
            "struct N{int value;struct N *next;};extern struct N b;struct N a={3,&b};struct N b={4,&a};",
            "struct N{\
                int value;\
                struct N *next;\
            };\
            extern struct N a,b;\
            int driver(void){\
                return a.next==&b&&b.next==&a&&a.next->value==4&&b.next->value==3;\
            }",
        ),
        (
            "integer_addresses",
            "int a[]={3,5,7};unsigned long n=(unsigned long)a+8;long m=(long)&a[1];",
            "extern int a[];\
            extern unsigned long n;\
            extern long m;\
            int driver(void){\
                return n==(unsigned long)&a[2]&&m==(long)&a[1]&&*(int *)n==7&&*(int *)m==5;\
            }",
        ),
        (
            "binary80_address",
            "long double a[]={1.25L,2.5L,7.75L};long double *p=a+2;",
            "extern long double a[];extern long double *p;int driver(void){return p==&a[2]&&*p==7.75L;}",
        ),
        (
            "external_array_addend",
            "extern int a[4];int *p=a+3;int (*row)[4]=&a;",
            "int a[4]={\
                3,5,7,11};\
            extern int *p;\
            extern int (*row)[4];\
            int driver(void){\
                return p==&a[3]&&*p==11&&row==&a&&(*row)[1]==5;\
            }",
        ),
        (
            "negative_integer_addend",
            "int x=7;unsigned long n=(unsigned long)&x-8;",
            "extern int x;extern unsigned long n;int driver(void){return n+8==(unsigned long)&x;}",
        ),
        (
            "conditional_array",
            "int a[]={3,7};const int b[]={5,9};const int *p=(1?a:b)+1;int *q=(1?a:(void*)0)+1;",
            "extern int a[];\
            extern const int *p;\
            extern int *q;\
            int driver(void){\
                return p==&a[1]&&q==&a[1]&&*p==7&&*q==7;\
            }",
        ),
        (
            "conditional_function",
            "int f(int x){return x+3;}int g(int x){return x+7;}int (*p)(int)=0?f:g;",
            "int g(int);extern int (*p)(int);int driver(void){return p==g&&p(2)==9;}",
        ),
    ] {
        pair(&mut cases, name, producer, consumer);
    }
    cases
}
