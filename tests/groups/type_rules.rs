// Type constraints and pointer arithmetic need width-sensitive behavioral checks.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (purpose, body, expected) in [
        ("const object read", "const int x=7;return x;", 7),
        (
            "volatile reads and stores",
            "volatile int x=1;x=3;x+=4;return x;",
            7,
        ),
        (
            "pointer to const",
            "const int x=7;const int *p=&x;return *p;",
            7,
        ),
        (
            "const pointer mutable target",
            "int x=1;int *const p=&x;*p=7;return x;",
            7,
        ),
        (
            "typedef const pointer",
            "typedef int *ptr;int x=1;const ptr p=&x;*p=7;return x;",
            7,
        ),
        (
            "prefix char preserves width",
            "unsigned char a[3];a[0]=3;a[1]=7;a[2]=9;++a[0];return a[0]+10*a[1]+a[2];",
            83,
        ),
        (
            "prefix short preserves width",
            "short a[3];a[0]=3;a[1]=7;a[2]=9;--a[0];return a[0]+10*a[1]+a[2];",
            81,
        ),
        (
            "positive pointer difference",
            "int a[8];return &a[7]-&a[2];",
            5,
        ),
        (
            "negative pointer difference",
            "double a[8];return &a[2]-&a[7];",
            251,
        ),
        ("array decay in difference", "int a[8];return &a[7]-a;", 7),
        (
            "pointer addition reverse",
            "int a[3];a[2]=7;return *(2+a);",
            7,
        ),
        (
            "constant expression dimension",
            "int a[2*3+1];return sizeof(a);",
            28,
        ),
        (
            "sizeof dimension",
            "char a[sizeof(long double)];return sizeof(a);",
            16,
        ),
        ("cast dimension", "int a[(int)3.9];return sizeof(a);", 12),
        (
            "compound integer floating conversion",
            "int a=3;a+=2.5;return a;",
            5,
        ),
    ] {
        cases.push(Case::new(
            format!("type_rules_{:03}", cases.len() + 1),
            purpose,
            format!("int main(void){{{body}}}"),
            Expectation::Exit(expected),
        ));
    }
    for (body, code) in [
        ("const int x=1;x=2;return x;", "E008"),
        ("const int x=1;x+=2;return x;", "E008"),
        ("const int x=1;++x;return x;", "E008"),
        ("const int x=1;x++;return x;", "E008"),
        ("int a=1,b=2;int *const p=&a;p=&b;return *p;", "E008"),
        ("int a=1;const int *p=&a;*p=2;return a;", "E008"),
        ("typedef int arr[2];const arr a;a[0]=1;return 0;", "E008"),
        ("int a[2];a=a;return 0;", "E008"),
        ("int a[2];a++;return 0;", "E008"),
        ("++1;return 0;", "E008"),
        ("int a[2];return a+a;", "E008"),
        ("int a[2];return 3-a;", "E008"),
        ("int a[2];return a+1.5;", "E008"),
        ("int a[2];return 1.5+a;", "E008"),
        ("int a[2];double b[2];return a-b;", "E008"),
        ("int a[3.5];return 0;", "E008"),
        ("int a[0];return 0;", "E008"),
        ("int a[-1];return 0;", "E008"),
        ("int n=3;int a[n];return 0;", "E008"),
    ] {
        cases.push(Case::new(
            format!("type_rules_{:03}", cases.len() + 1),
            body,
            format!("int main(void){{{body}}}"),
            Expectation::Reject(code.into()),
        ));
    }
    cases.push(Case::new(
        format!("type_rules_{:03}", cases.len() + 1),
        "parameter and outer block share scope",
        "int f(int x){int x=3;return x;}int main(void){return f(1);}",
        Expectation::Reject("E009".into()),
    ));
    cases
}
