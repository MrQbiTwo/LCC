// Values cases for aggregate.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, expected) in [
        (
            "member load and store",
            "struct S{int a;int b;};int main(void){struct S s;s.a=3;s.b=4;return s.a+s.b;}",
            7,
        ),
        (
            "internal and tail padding",
            "struct S{char a;long b;char c;};int main(void){return sizeof(struct S);}",
            24,
        ),
        (
            "field offsets",
            "struct S{\
                char a;\
                long b;\
                char c;\
            };\
            int main(void){\
                struct S s;\
                return (char*)&s.b-(char*)&s+10*((char*)&s.c-(char*)&s);\
            }",
            168,
        ),
        (
            "union maximum size and alignment",
            "union U{char a[9];long b;};int main(void){return sizeof(union U);}",
            16,
        ),
        (
            "union overlapping storage",
            "union U{\
                unsigned int a;\
                unsigned char b[4];\
            };\
            int main(void){\
                union U u;\
                u.a=0x04030201;\
                return u.b[0]+u.b[1]+u.b[2]+u.b[3];\
            }",
            10,
        ),
        (
            "arrow member access",
            "struct S{int n;};int main(void){struct S s;struct S *p=&s;p->n=7;return s.n;}",
            7,
        ),
        (
            "nested aggregate",
            "struct S{\
                char c;\
                struct N{\
                    short x;\
                    int y;\
                } \
                n;\
            };\
            int main(void){\
                struct S s;\
                s.n.x=3;\
                s.n.y=4;\
                return s.n.x+s.n.y+sizeof(s);\
            }",
            19,
        ),
        (
            "array of structures",
            "struct S{char c;long x;};int main(void){struct S a[3];a[2].x=7;return a[2].x+sizeof(a);}",
            55,
        ),
        (
            "member array decay",
            "struct S{\
                int a[3];\
            };\
            int f(int *p){\
                return p[2];\
            }\
            int main(void){\
                struct S s;\
                s.a[2]=7;\
                return f(s.a);\
            }",
            7,
        ),
        (
            "self referential pointer",
            "struct N{\
                int x;\
                struct N *next;\
            };\
            int main(void){\
                struct N a,b;\
                a.x=3;\
                b.x=4;\
                a.next=&b;\
                return a.x+a.next->x;\
            }",
            7,
        ),
        (
            "forward declaration completion",
            "struct S;struct S *p;struct S{int x;};int main(void){struct S s;p=&s;p->x=7;return s.x;}",
            7,
        ),
        (
            "qualified forward typedef completion",
            "typedef const struct S CS;\
            typedef volatile CS VCS;\
            struct S{\
                long x;\
            };\
            int main(void){\
                return sizeof(VCS);\
            }",
            8,
        ),
        (
            "qualified forward member read",
            "const struct S *p;struct S{int x;};int main(void){struct S s;s.x=7;p=&s;return p->x;}",
            7,
        ),
        (
            "anonymous typedef",
            "typedef struct{int x;} S;int main(void){S s;s.x=7;return s.x;}",
            7,
        ),
        (
            "comma field declarators",
            "struct S{int a,b,*p;};int main(void){struct S s;s.a=3;s.b=4;s.p=&s.a;return *s.p+s.b;}",
            7,
        ),
        (
            "global zero initialization",
            "struct S{int x;long y;};struct S s;int main(void){return s.x+s.y;}",
            0,
        ),
        (
            "aggregate assignment",
            "struct S{char a;long b;};int main(void){struct S a,b;a.a=3;a.b=4;b=a;return b.a+b.b;}",
            7,
        ),
        (
            "aggregate local initializer",
            "struct S{\
                int a;\
                double b;\
            };\
            int main(void){\
                struct S a;\
                a.a=3;\
                a.b=4.0;\
                struct S b=a;\
                return b.a+(int)b.b;\
            }",
            7,
        ),
        (
            "aggregate copy includes array members",
            "struct S{int a[4];};int main(void){struct S a,b;a.a[3]=7;b=a;a.a[3]=9;return b.a[3];}",
            7,
        ),
        (
            "chained aggregate assignment",
            "struct S{int a;};int main(void){struct S a,b,c;a.a=7;c=b=a;return c.a;}",
            7,
        ),
        (
            "aggregate assignment value member",
            "struct S{int a;};int main(void){struct S a,b;a.a=7;return (b=a).a;}",
            7,
        ),
        (
            "aggregate conditional value",
            "struct S{int a;};int main(void){struct S a,b,c;a.a=7;b.a=9;c=1?a:b;return c.a;}",
            7,
        ),
        (
            "aggregate comma value",
            "struct S{int a;};int main(void){struct S a,b;a.a=7;b=(1,a);return (b,7);}",
            7,
        ),
        (
            "union assignment",
            "union U{long x;double d;};int main(void){union U a,b;a.d=7.0;b=a;return (int)b.d;}",
            7,
        ),
        (
            "single byte struct preserves address",
            "struct S{char c;};int main(void){struct S a,b;a.c=7;b=a;return b.c;}",
            7,
        ),
        (
            "nested structure assignment",
            "struct S{\
                int a;\
            };\
            struct T{\
                char c;\
                struct S s;\
            };\
            int main(void){\
                struct T a,b;\
                a.s.a=7;\
                b.s=a.s;\
                return b.s.a;\
            }",
            7,
        ),
        (
            "const source is copyable",
            "struct S{\
                int a;\
            };\
            struct S s;\
            int main(void){\
                s.a=7;\
                const struct S *p=&s;\
                struct S t=*p;\
                return t.a;\
            }",
            7,
        ),
        (
            "nonconst field in const member aggregate",
            "struct S{const int a;int b;};int main(void){struct S s;s.b=7;return s.b;}",
            7,
        ),
        (
            "double and extended member alignment",
            "struct S{\
                char c;\
                double d;\
                long double l;\
                char e;\
            };\
            int main(void){\
                struct S s;\
                s.d=3.0;\
                s.l=4.0L;\
                return sizeof(s)+(int)(s.d+s.l);\
            }",
            55,
        ),
        (
            "postfix member evaluated once",
            "struct S{\
                int x;\
            };\
            int main(void){\
                struct S a[2];\
                struct S *p=a;\
                a[0].x=6;\
                a[1].x=99;\
                p++->x++;\
                return a[0].x+(p==a+1);\
            }",
            8,
        ),
        (
            "member function pointer",
            "int f(int x){\
                return x+1;\
            }\
            struct S{\
                int (*f)(int);\
            };\
            int main(void){\
                struct S s;\
                s.f=f;\
                return s.f(6);\
            }",
            7,
        ),
        (
            "local tag shadows global",
            "struct S{int x;};int main(void){struct S{long x;char c;};return sizeof(struct S);}",
            16,
        ),
        (
            "tag scope restoration",
            "struct S{int x;};int main(void){{struct S{long x;};}return sizeof(struct S);}",
            4,
        ),
        (
            "local forward shadows global",
            "struct S{int x;};int main(void){struct S;struct S{long x;};return sizeof(struct S);}",
            8,
        ),
        (
            "tag and ordinary name coexist",
            "struct S{int S;};int S;int main(void){struct S s;s.S=7;return s.S;}",
            7,
        ),
        (
            "complete pointer subtraction",
            "struct S{long x;char c;};int main(void){struct S a[4];return &a[3]-&a[0];}",
            3,
        ),
        (
            "array arrow decay",
            "struct S{int x;};int main(void){struct S a[2];a->x=7;return a[0].x;}",
            7,
        ),
        (
            "self assignment",
            "struct S{int x;};int main(void){struct S a;a.x=7;a=a;return a.x;}",
            7,
        ),
    ] {
        cases.push(Case::new(
            format!("aggregate_{:03}",cases.len()+1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
}
