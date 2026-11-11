// Operations cases for bitfield.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, purpose, source) in [
        (
            "truncate",
            "assignment expression yields the stored truncated value",
            "struct S{\
                unsigned a:3,b:5;\
            };\
            int main(void){\
                struct S s;\
                int x;\
                s.b=19;\
                x=(s.a=14);\
                return x==6&&s.a==6&&s.b==19;\
            }",
        ),
        (
            "signed_value",
            "signed assignment expression retains its narrow sign",
            "struct S{\
                signed int a:5;\
                unsigned b:3;\
            };\
            int main(void){\
                struct S s;\
                long x;\
                s.b=7;\
                x=(s.a=-13);\
                return x==-13&&s.a==-13&&s.b==7;\
            }",
        ),
        (
            "compound",
            "arithmetic compound stores truncate and preserve neighboring bits",
            "struct S{\
                unsigned a:3,b:5;\
            };\
            int main(void){\
                struct S s;\
                int x;\
                s.a=6;\
                s.b=23;\
                x=(s.a+=3);\
                if(x!=1)return 0;\
                x=(s.a*=6);\
                if(x!=6)return 0;\
                x=(s.a-=7);\
                return x==7&&s.a==7&&s.b==23;\
            }",
        ),
        (
            "compound_bits",
            "bitwise and shift compounds operate on the selected field",
            "struct S{\
                unsigned a:7,b:9;\
            };\
            int main(void){\
                struct S s;\
                int x;\
                s.a=99;\
                s.b=333;\
                s.a&=31;\
                s.a|=16;\
                s.a^=7;\
                s.a<<=2;\
                s.a>>=1;\
                x=(s.a%=7);\
                return x==5&&s.a==5&&s.b==333;\
            }",
        ),
        (
            "signed_divide",
            "signed compound division uses sign-extended field value",
            "struct S{\
                signed int a:6;\
                unsigned b:2;\
            };\
            int main(void){\
                struct S s;\
                s.a=-23;\
                s.b=3;\
                return (s.a/=4)==-5&&s.a==-5&&s.b==3;\
            }",
        ),
        (
            "increments",
            "prefix and postfix return distinct values at unsigned wrap",
            "struct S{\
                unsigned a:3,b:5;\
            };\
            int main(void){\
                struct S s;\
                int x,y,z;\
                s.a=7;\
                s.b=17;\
                x=s.a++;\
                y=++s.a;\
                z=s.a--;\
                return x==7&&y==1&&z==1&&s.a==0&&--s.a==7&&s.b==17;\
            }",
        ),
        (
            "signed_increment",
            "signed increment preserves negative expression results",
            "struct S{\
                signed int a:5;\
            };\
            int main(void){\
                struct S s;\
                int x,y;\
                s.a=-3;\
                x=s.a++;\
                y=++s.a;\
                return x==-3&&y==-1&&s.a==-1;\
            }",
        ),
        (
            "single_evaluation",
            "side-effecting member bases are evaluated once per update",
            "struct S{\
                unsigned a:4;\
            };\
            struct S s;\
            int count;\
            struct S *pick(void){\
                count++;\
                return &s;\
            }\
            int main(void){\
                int x;\
                s.a=5;\
                x=(pick()->a+=3);\
                if(x!=8)return 0;\
                x=pick()->a++;\
                return x==8&&s.a==9&&count==2;\
            }",
        ),
        (
            "promotions",
            "narrow unsigned fields promote to signed int",
            "struct S{\
                unsigned a:31,b:1;\
            };\
            int main(void){\
                struct S s;\
                s.a=2147483647;\
                s.b=1;\
                return -s.a<0&&~s.b==-2&&(s.a< -1)==0&&sizeof(+s.a)==4;\
            }",
        ),
        (
            "promote_division",
            "usual arithmetic conversions use bit-field width",
            "struct S{\
                unsigned a:4;\
            };\
            int main(void){\
                struct S s;\
                s.a=7;\
                return s.a/-2==-3&&s.a%-2==1&&(s.a< -1)==0;\
            }",
        ),
        (
            "plain_signed",
            "plain int fields use the signed implementation choice",
            "struct S{int a:5;};int main(void){struct S s={-7};return s.a==-7&&(s.a=-3)==-3;}",
        ),
        (
            "shift_promotions",
            "right operand does not change the promoted left shift type",
            "struct S{\
                signed int a:5;\
                unsigned b:4;\
            };\
            int main(void){\
                struct S s={\
                    -7,15};\
                return (s.a>>1U)==-4&&(s.b<<1UL)==30&&sizeof(s.b<<1UL)==4&&(s.a>>=1U)==-4;\
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
