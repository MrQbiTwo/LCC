// Accepted cases for constant expression.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "enum_integer_arithmetic",
            "integer arithmetic and bit operations remain valid enum constants",
            "enum E{N=((27/4+27%4)*3 ^ 5) & 31};int main(void){return N!=30;}",
        ),
        (
            "enum_relations",
            "integer comparisons and logical operators produce integer constants",
            "enum E{N=(3<5)+(7>=7)*2+(4!=5)*4+(2&&3)*8+(0||4)*16};int main(void){return N!=31;}",
        ),
        (
            "enum_references",
            "previous enumerators remain eligible through casts and arithmetic",
            "enum E{A=3,B=(unsigned char)A+4,C=B*2};int main(void){return C!=14;}",
        ),
        (
            "enum_character",
            "character constants participate in integer constant arithmetic",
            "enum E{N=('a'=='a')+('\\n'!='\\0')*2};int main(void){return N!=3;}",
        ),
        (
            "enum_sizeof_type",
            "sizeof a complete type is an integer constant operand",
            "enum E{N=sizeof(long)+sizeof(int)+sizeof(short)};int main(void){return N!=14;}",
        ),
        (
            "enum_float_literal",
            "an immediate floating literal may be cast to an integer",
            "enum E{N=(int)7.875};int main(void){return N!=7;}",
        ),
        (
            "enum_parenthesized_literal",
            "parentheses preserve immediate float-literal cast eligibility",
            "enum E{N=(int)(((6.875)))};int main(void){return N!=6;}",
        ),
        (
            "enum_float_suffix",
            "a float literal may be the immediate operand of an integer cast",
            "enum E{N=(unsigned char)9.75f};int main(void){return N!=9;}",
        ),
        (
            "enum_extended_suffix",
            "a long double literal may be immediately cast to an integer",
            "enum E{N=(long)11.75L};int main(void){return N!=11;}",
        ),
        (
            "enum_negate_integer_cast",
            "unary negation after the float-to-integer cast is valid",
            "enum E{N=-(int)7.875};int main(void){return N!=-7;}",
        ),
        (
            "enum_nested_integer_cast",
            "outer integer casts retain the validity of an immediate float cast",
            "enum E{N=(short)(unsigned int)13.75};int main(void){return N!=13;}",
        ),
        (
            "enum_unsigned_wrap",
            "defined unsigned wrapping is valid before conversion to the enum value",
            "enum E{N=(0xffffffffU+3U)};int main(void){return N!=2;}",
        ),
        (
            "enum_skipped_division",
            "an unselected integer division by zero is not evaluated",
            "enum E{N=1?7:1/0};int main(void){return N!=7;}",
        ),
        (
            "enum_skipped_overflow",
            "an unselected signed overflow expression is not evaluated",
            "enum E{N=0?2147483647+1:9};int main(void){return N!=9;}",
        ),
        (
            "enum_short_circuit",
            "logical operators skip invalid arithmetic without weakening operand rules",
            "enum E{A=0&&(1/0),B=1||(2147483647+1)};int main(void){return !(A==0&&B==1);}",
        ),
        (
            "enum_sizeof_call",
            "sizeof a call is constant and does not call the function",
            "int n;\
            int f(void){\
                n++;\
                return 1;\
            }\
            enum E{\
                N=sizeof(f())};\
            int main(void){\
                return !(N==4&&n==0);\
            }",
        ),
        (
            "enum_sizeof_assignment",
            "sizeof permits assignment and leaves the object unchanged",
            "int n;enum E{N=sizeof(n=7)};int main(void){return !(N==4&&n==0);}",
        ),
        (
            "enum_sizeof_comma_increment",
            "sizeof shields comma and increment operands from evaluation",
            "int n;enum E{N=sizeof((n++,n+=2))};int main(void){return !(N==4&&n==0);}",
        ),
        (
            "enum_sizeof_prefix",
            "sizeof shields prefix increment and decrement from evaluation",
            "int n;enum E{N=sizeof(++n)+sizeof(--n)};int main(void){return !(N==8&&n==0);}",
        ),
        (
            "array_file_arithmetic",
            "file array bounds admit enum and integer arithmetic operands",
            "enum E{N=3};int a[(N<<1)+1];int main(void){return sizeof(a)!=28;}",
        ),
        (
            "array_local_float_cast",
            "a block array bound accepts an immediate float-to-int cast",
            "int main(void){int a[(int)3.875];return sizeof(a)!=12;}",
        ),
        (
            "array_dimensions",
            "each multidimensional array bound is checked independently",
            "int a[(int)2.75][sizeof(short)+1];int main(void){return sizeof(a)!=24;}",
        ),
        (
            "bitfield_float_cast",
            "bit-field width accepts an immediate floating literal cast",
            "struct S{unsigned n:(int)3.75;};int main(void){struct S s;s.n=7;return s.n!=7;}",
        ),
        (
            "bitfield_sizeof",
            "bit-field width accepts sizeof an integer type",
            "struct S{unsigned n:sizeof(short)+1;};int main(void){struct S s;s.n=5;return s.n!=5;}",
        ),
        (
            "case_float_cast",
            "a switch label accepts a directly converted float literal",
            "int main(void){switch(5){case (int)5.75:return 0;default:return 1;}}",
        ),
        (
            "case_sizeof_assignment",
            "sizeof a side effect remains valid in a switch label",
            "int main(void){int n=0;switch(4){case sizeof(n=3):return n;default:return 1;}}",
        ),
        (
            "offset_float_cast",
            "an offsetof array index accepts a directly converted floating literal",
            "#include <stddef.h>\n\
            struct S{\
                char c;\
                int a[4];\
            };\
            int main(void){\
                return offsetof(struct S,a[(int)2.75])!=12;\
            }",
        ),
        (
            "offset_sizeof_assignment",
            "sizeof shields side effects in an offsetof array index",
            "#include <stddef.h>\n\
            struct S{\
                int a[6];\
            };\
            int main(void){\
                int n=0;\
                return !(offsetof(struct S,a[sizeof(n=3)])==16&&n==0);\
            }",
        ),
        (
            "arithmetic_contexts",
            "static and runtime arithmetic keep their broader floating-expression rules",
            "int a=(int)(1.5+2.5);\
            int b=1.5<2.5;\
            int c=!!2.5;\
            int main(void){\
                double x=2.5;\
                return !(a==4&&b==1&&c==1&&(int)(x+1.5)==4);\
            }",
        ),
        (
            "null_literal_cast",
            "an immediate zero float-to-integer cast is a null pointer constant",
            "int x=7;int *p=1?&x:(int)0.0;int main(void){return p!=&x;}",
        ),
        (
            "null_negated_cast",
            "negating a valid integer cast preserves the null pointer constant",
            "int x=7;int *p=0?-(int)0.0:&x;int main(void){return p!=&x;}",
        ),
    ] {
        cases.push(Case::new(
            format!("constant_expression_{}", id),
            purpose,
            source,
            Expectation::Exit(0),
        ));
    }
}
