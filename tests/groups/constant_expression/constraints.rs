// Constraints cases for constant expression.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // All five consumers must classify the complete expression before folding it.
    for (form, expression, reason) in [
        (
            "float_arithmetic",
            "(int)(1.0+2.0)",
            "floating arithmetic is not an immediate literal cast",
        ),
        (
            "float_comparison",
            "1.0<2.0",
            "floating comparison operands are not integer constant operands",
        ),
        (
            "float_truth",
            "!!1.0",
            "floating truth conversion does not make an integer constant expression",
        ),
        (
            "pointer_cast",
            "(int)(void*)3",
            "a pointer cast is not permitted in an integer constant expression",
        ),
        (
            "variable",
            "n",
            "an initialized object is not an integer constant operand",
        ),
    ] {
        for (context, source) in [
            (
                "enum",
                format!("int n=2;enum E{{N={expression}}};int main(void){{return 0;}}"),
            ),
            (
                "array",
                format!("int n=2;int a[{expression}];int main(void){{return 0;}}"),
            ),
            (
                "bitfield",
                format!("int n=2;struct S{{unsigned member:{expression};}};int main(void){{return 0;}}"),
            ),
            (
                "case",
                format!("int n=2;int main(void){{switch(3){{case {expression}:return 0;default:return 1;}}}}"),
            ),
            (
                "offset",
                format!("#include <stddef.h>\n\
                int n=2;\
                struct S{{\
                    int a[8];\
                }};\
                int main(void){{\
                    return offsetof(struct S,a[{expression}]);\
                }}"),
            ),
        ] {
            cases.push(Case::new(
                format!("constant_expression_{}", &format!("reject_{context}_{form}")),
                reason,
                &source,
                Expectation::Reject("E008".into()),
            ));
        }
    }
    for (id, purpose, source) in [
        (
            "reject_negative_literal_cast",
            "unary minus separates a floating literal from its integer cast",
            "enum E{N=(int)-3.5};int main(void){return 0;}",
        ),
        (
            "reject_positive_literal_cast",
            "unary plus separates a floating literal from its integer cast",
            "enum E{N=(int)+3.5};int main(void){return 0;}",
        ),
        (
            "reject_float_cast_chain",
            "an intermediate floating cast is not a literal operand",
            "enum E{N=(int)(float)3.5};int main(void){return 0;}",
        ),
        (
            "reject_integer_to_float_chain",
            "casting an integer through floating type is not an integer constant expression",
            "enum E{N=(int)(double)3};int main(void){return 0;}",
        ),
        (
            "reject_float_conditional",
            "integer conversion of a floating conditional is not an immediate literal cast",
            "enum E{N=(int)(1?2.0:3.0)};int main(void){return 0;}",
        ),
        (
            "reject_comma",
            "an evaluated comma operator is not valid in an integer constant expression",
            "enum E{N=(1,2)};int main(void){return 0;}",
        ),
        (
            "reject_skipped_call",
            "an unselected call still disqualifies an integer constant expression",
            "int f(void){return 2;}enum E{N=1?3:f()};int main(void){return 0;}",
        ),
        (
            "reject_skipped_variable",
            "short-circuit evaluation does not admit variable operands",
            "int n;enum E{N=0&&n};int main(void){return 0;}",
        ),
        (
            "reject_skipped_assignment",
            "short-circuit evaluation does not admit assignment operands",
            "int n;enum E{N=1||(n=2)};int main(void){return 0;}",
        ),
        (
            "reject_skipped_prefix_increment",
            "an unselected prefix increment is not an integer constant operand",
            "int n;enum E{N=1?3:++n};int main(void){return 0;}",
        ),
        (
            "reject_skipped_prefix_decrement",
            "short-circuit evaluation does not admit prefix decrement",
            "int n;enum E{N=0&&--n};int main(void){return 0;}",
        ),
        (
            "reject_skipped_postfix_increment",
            "short-circuit evaluation does not admit postfix increment",
            "int n;enum E{N=1||n++};int main(void){return 0;}",
        ),
        (
            "reject_skipped_postfix_decrement",
            "an unselected postfix decrement is not an integer constant operand",
            "int n;enum E{N=1?3:n--};int main(void){return 0;}",
        ),
        (
            "reject_skipped_float",
            "a skipped floating operand remains outside the integer constant subset",
            "enum E{N=0&&1.0};int main(void){return 0;}",
        ),
        (
            "reject_skipped_float_arithmetic",
            "the unselected conditional arm must also obey operand restrictions",
            "enum E{N=1?3:(int)(1.0+2.0)};int main(void){return 0;}",
        ),
        (
            "reject_null_float_arithmetic",
            "foldable floating arithmetic does not supply an integer null pointer constant",
            "int x=7;int *p=1?&x:(int)(0.0+0.0);int main(void){return 0;}",
        ),
    ] {
        cases.push(Case::new(
            format!("constant_expression_{}", id),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
    for (id, purpose, source) in [
        (
            "pp_skipped_division",
            "preprocessing logical evaluation skips division by zero",
            "#if !(1 || (1/0))\n#error logical result\n#endif\nint main(void){return 0;}",
        ),
        (
            "pp_skipped_conditional",
            "preprocessing conditional evaluation skips the unselected invalid arithmetic",
            "#if !(0 ? (1/0) : 1)\n#error conditional result\n#endif\nint main(void){return 0;}",
        ),
        (
            "pp_macro_defined_character",
            "macro expansion, defined and character constants retain integer expression eligibility",
            "#define N 3\n\
            #if !defined(N) || N*2 != 6 || ('a' != 'a') || defined(MISSING)\n\
            #error macro result\n\
            #endif\n\
            int main(void){\
                return 0;\
            }",
        ),
    ] {
        cases.push(Case::new(format!("constant_expression_{}", id), purpose, source, Expectation::Exit(0)));
    }
    for (id, purpose, expression) in [
        (
            "reject_pp_skipped_comma",
            "preprocessing rejects comma even in a skipped operand",
            "1 || (1,2)",
        ),
        (
            "reject_pp_skipped_string",
            "preprocessing rejects a string even in a skipped operand",
            "0 && \"x\"",
        ),
        (
            "reject_pp_comma",
            "preprocessing rejects an evaluated comma operator",
            "(1,2)",
        ),
    ] {
        cases.push(Case::new(
            format!("constant_expression_{}", id),
            purpose,
            &format!("#if {expression}\nint value;\n#endif\nint main(void){{return 0;}}"),
            Expectation::Reject("E013".into()),
        ));
    }
}
