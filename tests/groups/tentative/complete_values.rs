// Complete values cases for tentative.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "complete_value_expressions",
            "completed records permit assignment and discarded value expressions",
            "struct S a,b;struct S{int n;};int main(void){b.n=113;a=b;a;(void)a;a,b;return a.n!=113;}",
        ),
        (
            "complete_call",
            "completed tentative records can be passed by value",
            "struct S object;\
            struct S{\
                long n;\
            };\
            long take(struct S x){\
                return x.n;\
            }\
            int main(void){\
                object.n=127;\
                return take(object)!=127;\
            }",
        ),
        (
            "complete_conditional",
            "completed records can be selected by a conditional expression",
            "struct S a,b;\
            struct S{\
                int n;\
            };\
            int main(void){\
                struct S c;\
                a.n=131;\
                b.n=137;\
                c=1?a:b;\
                return c.n!=131;\
            }",
        ),
        (
            "complete_for",
            "completed record expressions are legal discarded loop clauses",
            "struct S object;\
            struct S{\
                int n;\
            };\
            int main(void){\
                int i=0;\
                for(object;i<2;object)i++;\
                return i!=2;\
            }",
        ),
        (
            "complete_return",
            "completed tentative records can be returned by value",
            "struct S object;\
            struct S{\
                long n;\
            };\
            struct S get(void){\
                return object;\
            }\
            int main(void){\
                struct S value;\
                object.n=139;\
                value=get();\
                return value.n!=139;\
            }",
        ),
        (
            "unused_extern",
            "incomplete declaration-only records need no definition",
            "extern struct S missing;extern const union U absent;int main(void){return 0;}",
        ),
        (
            "unused_block_extern",
            "unused block extern incomplete records remain declaration-only",
            "struct S;\
            union U;\
            int main(void){\
                extern volatile struct S missing;\
                extern const union U absent;\
                return 0;\
            }",
        ),
        (
            "ordinary_tentative_array",
            "an ordinary unknown-bound tentative array still gains one element",
            "int values[];int main(void){values[0]=107;return values[0]!=107;}",
        ),
        (
            "late_array_bound",
            "a later complete array declaration still sets tentative storage size",
            "int values[];\
            int values[3];\
            int main(void){\
                values[2]=109;\
                return !(sizeof(values)==12&&values[0]==0&&values[1]==0&&values[2]==109);\
            }",
        ),
    ] {
        cases.push(Case::new(
format!("tentative_{id}"),
purpose,
source,
Expectation::Exit(0),
));
    }
}
