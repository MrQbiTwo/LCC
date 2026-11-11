// Layout cases for bitfield.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, purpose, source) in [
        (
            "unsigned",
            "adjacent fields preserve each other",
            "struct S{\
                unsigned a:3,b:5,c:24;\
            };\
            int main(void){\
                struct S s;\
                s.a=5;\
                s.b=29;\
                s.c=0xabcdef;\
                return s.a==5&&s.b==29&&s.c==0xabcdef&&sizeof(s)==4;\
            }",
        ),
        (
            "signed",
            "signed fields sign extend independently",
            "struct S{\
                signed int a:1,b:5,c:26;\
            };\
            int main(void){\
                struct S s;\
                s.a=-1;\
                s.b=-13;\
                s.c=-1234567;\
                return s.a==-1&&s.b==-13&&s.c==-1234567;\
            }",
        ),
        (
            "full_width",
            "32-bit unsigned masks and sign extension",
            "struct S{\
                unsigned a:32;\
                signed int b:32;\
            };\
            int main(void){\
                struct S s;\
                s.a=4294967295U;\
                s.b=-2147483647-1;\
                return s.a==4294967295U&&s.a>0&&s.b==(-2147483647-1)&&sizeof(s)==8;\
            }",
        ),
        (
            "share_char",
            "ordinary bytes share a declared int storage unit",
            "struct S{\
                char c;\
                unsigned a:4,b:12;\
                char tail;\
            };\
            int main(void){\
                struct S s;\
                s.c=18;\
                s.a=5;\
                s.b=0xabc;\
                s.tail=52;\
                return sizeof(s)==4&&(char*)&s.tail-(char*)&s==3&&s.c==18&&s.a==5&&s.b==0xabc&&s.tail==52;\
            }",
        ),
        (
            "unit_boundary",
            "a field cannot cross its aligned int storage unit",
            "struct S{\
                char c;\
                unsigned a:28,b:4;\
                char tail;\
            };\
            int main(void){\
                struct S s;\
                s.c=7;\
                s.a=0xabcdef;\
                s.b=13;\
                s.tail=9;\
                return sizeof(s)==12&&(char*)&s.tail-(char*)&s==8&&s.c==7&&s.a==0xabcdef&&s.b==13&&\
                s.tail==9;\
            }",
        ),
        (
            "zero_width",
            "zero width advances allocation without raising aggregate alignment",
            "struct S{\
                char a;\
                unsigned :0;\
                char b;\
            };\
            struct T{\
                char a;\
                struct S s;\
            };\
            int main(void){\
                struct S s;\
                struct T t;\
                return sizeof(s)==5&&(char*)&s.b-(char*)&s==4&&(char*)&t.s-(char*)&t==1;\
            }",
        ),
        (
            "unnamed",
            "unnamed nonzero fields consume bits without imposing alignment",
            "struct S{\
                unsigned :3;\
                char c;\
            };\
            struct T{\
                char a;\
                struct S s;\
            };\
            int main(void){\
                struct S s;\
                struct T t;\
                s.c=7;\
                return sizeof(s)==2&&(char*)&s.c-(char*)&s==1&&(char*)&t.s-(char*)&t==1&&s.c==7;\
            }",
        ),
        (
            "split_run",
            "zero-width field splits named allocation runs",
            "struct S{\
                unsigned a:3;\
                unsigned :0;\
                unsigned b:3;\
                char c;\
            };\
            int main(void){\
                struct S s;\
                s.a=5;\
                s.b=6;\
                s.c=9;\
                return sizeof(s)==8&&(char*)&s.c-(char*)&s==5&&s.a==5&&s.b==6&&s.c==9;\
            }",
        ),
        (
            "tail_stride",
            "a partial final unit retains named int alignment",
            "struct S{\
                unsigned a:4;\
                char b;\
            };\
            int main(void){\
                struct S s[2];\
                s[0].a=3;\
                s[0].b=4;\
                s[1].a=5;\
                s[1].b=6;\
                return sizeof(s)==8&&(char*)&s[1]-(char*)&s[0]==4&&s[0].a==3&&s[0].b==4&&s[1].a==5&&\
                s[1].b==6;\
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
