// Initialization cases for bitfield.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, purpose, source) in [
        (
            "scalar_braces",
            "scalar braces retain field metadata",
            "struct S{\
                unsigned a:3,b:5;\
            };\
            struct S g={{\
                {13}},{{\
                19}}\
            };\
            int main(void){\
                struct S s={{\
                    {14}},{{\
                    23}}\
            };\
            return g.a==5&&g.b==19&&s.a==6&&s.b==23;\
            }",
        ),
        (
            "zero_width_union",
            "unnamed zero-width union member changes no alignment or storage",
            "union U{\
                unsigned :0;\
                char c;\
            };\
            struct S{\
                char c;\
                union U u;\
            };\
            int main(void){\
                union U u={7};\
                struct S s;\
                return sizeof(u)==1&&sizeof(s)==2&&(char*)&s.u-(char*)&s==1&&u.c==7;\
            }",
        ),
        (
            "float_conversion",
            "floating assignment converts before field truncation",
            "struct S{\
                unsigned a:4;\
                signed int b:5;\
            };\
            int main(void){\
                struct S s;\
                s.a=21.75;\
                s.b=-13.75;\
                return s.a==5&&s.b==-13&&(double)s.b==-13.0;\
            }",
        ),
        (
            "conditional",
            "conditional and comma value uses preserve promotion",
            "struct S{\
                unsigned a:3,b:3;\
            };\
            int main(void){\
                struct S s;\
                s.a=5;\
                s.b=2;\
                return (1?s.a:s.b)==5&&(0,s.a)==5&&-(1?s.a:s.b)==-5;\
            }",
        ),
        (
            "union",
            "union bit-fields begin at bit zero",
            "union U{\
                unsigned a:3;\
                unsigned b:7;\
                unsigned whole;\
            };\
            int main(void){\
                union U u;\
                u.whole=0;\
                u.b=85;\
                return sizeof(u)==4&&u.a==5&&u.b==85&&u.whole==85;\
            }",
        ),
        (
            "nested_copy",
            "aggregate copy retains nested bit-fields and array strides",
            "struct S{\
                signed int a:5;\
                unsigned b:3;\
            };\
            struct T{\
                char c;\
                struct S s[2];\
            };\
            int main(void){\
                struct T a,b;\
                a.c=9;\
                a.s[0].a=-7;\
                a.s[0].b=5;\
                a.s[1].a=11;\
                a.s[1].b=6;\
                b=a;\
                a.s[0].a=0;\
                return sizeof(a)==12&&b.c==9&&b.s[0].a==-7&&b.s[0].b==5&&b.s[1].a==11&&b.s[1].b==6;\
            }",
        ),
        (
            "constant_width",
            "integer constant expressions and typedefs declare fields",
            "enum E{\
                W=3};\
            typedef unsigned int U;\
            struct S{\
                U a:W+2,b:(sizeof(int)*8-5);\
            };\
            int main(void){\
                struct S s;\
                s.a=27;\
                s.b=123456;\
                return sizeof(s)==4&&s.a==27&&s.b==123456;\
            }",
        ),
        (
            "const_read",
            "qualified aggregate and field reads preserve signedness",
            "struct S{\
                const signed int a:5;\
                unsigned b:3;\
            };\
            int main(void){\
                const struct S s={\
                    -7,5};\
                return s.a==-7&&s.b==5;\
            }",
        ),
        (
            "volatile",
            "volatile read-modify-write preserves neighboring fields",
            "struct S{\
                volatile unsigned a:3,b:5;\
            };\
            int main(void){\
                volatile struct S s;\
                s.a=6;\
                s.b=19;\
                s.a++;\
                return s.a==7&&s.b==19;\
            }",
        ),
        (
            "local_init",
            "initializers skip unnamed fields and zero omitted named fields",
            "struct S{\
                unsigned a:3;\
                unsigned :2;\
                signed int b:5;\
                unsigned :0;\
                unsigned c:4,d:4;\
            };\
            int main(void){\
                struct S s={\
                    5,-7,9};\
                return s.a==5&&s.b==-7&&s.c==9&&s.d==0&&sizeof(s)==8;\
            }",
        ),
        (
            "global_init",
            "global constants pack adjacent fields and keep ordinary bytes",
            "struct S{\
                char c;\
                unsigned a:4,b:12;\
                char tail;\
            };\
            struct S s={\
                18,21,0xabc,52};\
            int main(void){\
                unsigned char *p=(unsigned char*)&s;\
                return s.c==18&&s.a==5&&s.b==0xabc&&s.tail==52&&p[0]==18&&p[1]==0xc5&&p[2]==0xab&&\
                p[3]==52;\
            }",
        ),
        (
            "partial_init",
            "partial bit-field initialization does not erase explicit neighbors",
            "struct S{\
                unsigned a:3,b:5;\
                char c;\
            };\
            struct S g={5};\
            int main(void){\
                struct S s={6};\
                return g.a==5&&g.b==0&&g.c==0&&s.a==6&&s.b==0&&s.c==0;\
            }",
        ),
        (
            "nested_init",
            "brace elision traverses nested bit-field aggregates",
            "struct S{\
                unsigned a:3;\
                unsigned :4;\
                signed int b:5;\
            };\
            struct T{\
                struct S s[2];\
                int n;\
            };\
            struct T g={\
                1,-3,2,-4,9};\
            int main(void){\
                struct T t={{ \
                    {\
                        3,-5},{\
                        4,-6} \
                },8};\
            return g.s[0].a==1&&g.s[0].b==-3&&g.s[1].a==2&&g.s[1].b==-4&&g.n==9&&t.s[0].a==3&&\
            t.s[1].b==-6&&t.n==8;\
            }",
        ),
        (
            "union_init",
            "union initialization chooses first named member",
            "union U{\
                unsigned :3;\
                signed int a:5;\
                long b;\
            };\
            union U g={-7};\
            int main(void){\
                union U u={-9};\
                return g.a==-7&&u.a==-9&&sizeof(u)==8;\
            }",
        ),
        (
            "initializer_effects",
            "adjacent initialized fields evaluate each expression once",
            "int n;\
            int next(void){\
                return ++n;\
            }\
            struct S{\
                unsigned a:3,b:3,c:3;\
            };\
            int main(void){\
                struct S s={\
                    next(),next(),next()};\
                return n==3&&s.a==1&&s.b==2&&s.c==3;\
            }",
        ),
    ] {
        cases.push(Case::new(
            format!("bitfield_{name}"),
            purpose,
            source,
            Expectation::Exit(1),
        ));
    }
}
