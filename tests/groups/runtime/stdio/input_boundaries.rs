// Input boundaries cases for runtime/stdio.
use super::io;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // A consumed incomplete numeric prefix is a matching failure; the next byte stays unread.
    for (id, format, input) in [
        ("sign", "%d", "+Q"),
        ("hex_prefix", "%i", "0xQ"),
        ("point", "%lf", ".Q"),
        ("exponent", "%lf", "1e+Q"),
    ] {
        let (declaration, print) = if format == "%lf" {
            ("double a=9.0", "%.0f")
        } else {
            ("int a=9", "%d")
        };
        cases.push(io(
            &format!("scan_prefix_{id}"),
            "incomplete numeric prefix consumes only its candidate bytes",
            &format!("{declaration};int n;n=scanf(\"{format}\",&a);printf(\"%d {print} %d\",n,a,getchar());return 0;"),
            input,
            "0 9 81".into(),
        ));
    }
    for (id, format) in [
        ("unknown", "%q"),
        ("width", "%4097d"),
        ("long_double", "%Lf"),
        ("repeat_modifier", "%lld"),
        ("wide_char", "%lc"),
        ("count_store", "%n"),
        ("trailing", "%"),
    ] {
        cases.push(io(
            &format!("scanf_reject_{id}"),
            "unsupported input format returns -1 without consuming input",
            &format!(
                "int a=9,n;n=scanf(\"{format}\",&a);printf(\"%d %d %d\",n,a,getchar());return 0;"
            ),
            "Q",
            "-1 9 81".into(),
        ));
    }
    cases.push(io(
        "input_refill",
        "whitespace skipping crosses repeated input-buffer refills",
        "int a,n;n=scanf(\"%d\",&a);printf(\"%d %d %d\",n,a,getchar());return 0;",
        &format!("{}123!", " ".repeat(9000)),
        "1 123 33".into(),
    ));
    cases.push(io(
        "string_width_limit",
        "maximum string width spans a full input buffer and preserves suffix",
        "char s[4097];\
        int n;\
        n=scanf(\"%4096s\",s);\
        printf(\"%d %d %d %d %d\",n,s[0],s[4095],s[4096],getchar());\
        return 0;",
        &format!("{}!", "a".repeat(4096)),
        "1 97 97 0 33".into(),
    ));
}
