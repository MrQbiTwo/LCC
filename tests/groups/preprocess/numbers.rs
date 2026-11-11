// Preprocessing numbers are converted only after expansion or directive evaluation.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (purpose, source, expected) in [
        (
            "unused malformed numeric macro",
            "#define UNUSED 1e+\nint main(void){return 7;}",
            7,
        ),
        (
            "inactive malformed numeric token",
            "#if 0\n1.5ff 1e+ 0x\n#endif\nint main(void){return 7;}",
            7,
        ),
        (
            "discarded macro argument",
            "#define FIRST(a,b) a\nint main(void){return FIRST(7,1e+);}",
            7,
        ),
        (
            "paste exponent",
            "#define CAT(a,b) a##b\nint main(void){return CAT(1e,2)==100.0;}",
            1,
        ),
        (
            "paste hexadecimal prefix",
            "#define CAT(a,b) a##b\nint main(void){return CAT(0x,ff);}",
            255,
        ),
        (
            "stringify preprocessing number",
            "#define STR(x) #x\nint main(void){return sizeof(STR(1e+));}",
            4,
        ),
        (
            "stringify overflowing integer",
            "#define STR(x) #x\nint main(void){return sizeof(STR(999999999999999999999999999999));}",
            31,
        ),
        (
            "conditional character constant",
            "#if 'A' == 65\nint main(void){return 7;}\n#else\n#error bad\n#endif",
            7,
        ),
        (
            "line number macro",
            "#define LINE 42\n#line LINE \"virtual.c\"\nint main(void){return 7;}",
            7,
        ),
        (
            "numeric macro used in condition and body",
            "#define VALUE 7\n#if VALUE==7\nint main(void){return VALUE;}\n#endif",
            7,
        ),
    ] {
        cases.push(Case::new(
            format!("pp_numbers_{:03}",cases.len()+1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
    for (source, code) in [
        ("#define BAD 1e+\nint main(void){return BAD;}", "E007"),
        ("#if 1e+\n#endif\nint main(void){return 0;}", "E007"),
        ("#line 1.5\nint main(void){return 0;}", "E013"),
        (
            "#line 42 \"virtual.c\"\nint main(void){return unknown;}",
            "virtual.c:42:",
        ),
    ] {
        cases.push(Case::new(
            format!("pp_numbers_{:03}", cases.len() + 1),
            source,
            source,
            Expectation::Reject(code.into()),
        ));
    }
    cases
}
