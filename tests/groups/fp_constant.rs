// Target-rounded constant evaluation and rejected arithmetic constraints.
use crate::{Case, Expectation};

// Constants are checked through global loads so runtime evaluation cannot hide errors.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (ty, suffix) in [("float", "f"), ("double", ""), ("long double", "L")] {
        for (expression, expected) in [
            (format!("7.5{suffix}+2.5{suffix}"), 10),
            (format!("7.5{suffix}-2.5{suffix}"), 5),
            (format!("7.5{suffix}*2.5{suffix}"), 18),
            (format!("7.5{suffix}/2.5{suffix}"), 3),
            (format!("-7.5{suffix}"), -7),
            (format!("7.5{suffix}+2"), 9),
            (format!("1 ? 3.5{suffix}:7"), 3),
            (format!("0 ? 3.5{suffix}:7"), 7),
            ("42".into(), 42),
            (format!("({ty})3.5L"), 3),
        ] {
            cases.push(Case::new(
                format!("fp_constant_{:03}", cases.len() + 1),
                format!("{ty} {expression}"),
                format!("{ty} x={expression}; int main(void){{return x;}}"),
                Expectation::Exit(expected & 255),
            ));
        }
    }
    for (expression, expected) in [
        ("1.5 < 2.5", 1),
        ("1.5 > 2.5", 0),
        ("1.5 <= 1.5", 1),
        ("1.5 >= 2.5", 0),
        ("1.5 == 1.5", 1),
        ("1.5 != 1.5", 0),
        ("1.5L < 2.5L", 1),
        ("1.5L > 2.5L", 0),
        ("!(-0.0)", 1),
        ("!!(-0.0L)", 0),
        ("(-0.0) && (1.0/0.0)", 0),
        ("1.0 || (1/0)", 1),
        ("(int)-7.9", -7),
        ("(unsigned int)-0.75", 0),
        ("(int)2147483647.9", 2147483647),
        ("(int)-2147483648.9", -2147483648),
        (
            "(unsigned long)18446744073709551615.0L == 18446744073709551615UL",
            1,
        ),
        (
            "(long)-9223372036854775808.0L == (-9223372036854775807L-1)",
            1,
        ),
        ("(long double)9007199254740993UL > 9007199254740992.0L", 1),
        ("(float)16777217 == 16777216.0f", 1),
    ] {
        cases.push(Case::new(
            format!("fp_constant_{:03}", cases.len() + 1),
            expression,
            format!("int x={expression}; int main(void){{return x;}}"),
            Expectation::Exit(expected & 255),
        ));
    }
    for (source, code) in [
        ("int main(void){return 1.0%2.0;}", "E008"),
        ("int main(void){return 1.0&2;}", "E008"),
        ("int main(void){return 1.0|2;}", "E008"),
        ("int main(void){return 1.0^2;}", "E008"),
        ("int main(void){return 1.0<<2;}", "E008"),
        ("int main(void){return 1.0>>2;}", "E008"),
        ("int main(void){return ~1.0;}", "E008"),
        ("int main(void){double x=1; x%=2; return 0;}", "E008"),
        ("int main(void){double x=1; x&=2; return 0;}", "E008"),
        ("int main(void){double x=1; x<<=2; return 0;}", "E008"),
        ("int main(void){return 1e500;}", "E011"),
        ("int main(void){return 1e50f;}", "E011"),
        ("int main(void){return 1e5000L;}", "E011"),
        ("int main(void){return 1.5ff;}", "E007"),
        ("int main(void){return 1e+;}", "E007"),
        ("double x=1.0/0.0; int main(void){return 0;}", "E011"),
        ("double x=1e300*1e300; int main(void){return 0;}", "E011"),
        ("float x=1e300; int main(void){return 0;}", "E011"),
        ("int x=(int)2147483648.0; int main(void){return 0;}", "E011"),
        (
            "int x=(int)-2147483649.0; int main(void){return 0;}",
            "E011",
        ),
        (
            "unsigned int x=(unsigned int)-1.0; int main(void){return 0;}",
            "E011",
        ),
        (
            "long x=(long)9223372036854775808.0L; int main(void){return 0;}",
            "E011",
        ),
        (
            "unsigned long x=(unsigned long)18446744073709551616.0L; int main(void){return 0;}",
            "E011",
        ),
        ("#if 1.0\n#endif\nint main(void){return 0;}", "E013"),
    ] {
        cases.push(Case::new(
            format!("fp_constant_{:03}", cases.len() + 1),
            source,
            source,
            Expectation::Reject(code.into()),
        ));
    }
    cases
}
