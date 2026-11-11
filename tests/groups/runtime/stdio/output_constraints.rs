// Output constraints cases for runtime/stdio.
use super::io;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Unsupported formats fail deterministically before their argument is fetched.
    for (id, format, args) in [
        ("unknown", "%q", ""),
        ("trailing_percent", "%", ""),
        ("exponential", "%e", ",1.0"),
        ("hexadecimal", "%x", ",1"),
        ("count_store", "%n", ",0"),
        ("long_double", "%Lf", ",1.0L"),
        ("long_string", "%ls", ",0"),
        ("short_character", "%hc", ",'x'"),
        ("repeat_modifier", "%lld", ",1L"),
        ("alternate_form", "%#d", ",1"),
        ("width_bound", "%4097d", ",1"),
        ("precision_bound", "%.4097f", ",1.0"),
        ("dynamic_width_bound", "%*d", ",4097,1"),
        ("minimum_width", "%*d", ",-2147483647-1,1"),
        ("dynamic_precision_bound", "%.*f", ",4097,1.0"),
    ] {
        cases.push(io(
            &format!("printf_reject_{id}"),
            "invalid format returns -1 after preceding literal output",
            &format!("int n;n=printf(\"prefix{format}\"{args});printf(\"|%d\",n);return 0;"),
            "",
            "prefix|-1".into(),
        ));
    }
}
