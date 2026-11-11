// Scoped aliases share the ordinary C identifier namespace without emitted objects.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (purpose, source, expected) in [
        (
            "integer alias",
            "typedef int count; int main(void){count n=7;return n;}",
            7,
        ),
        (
            "unsigned alias",
            "typedef unsigned long word; int main(void){word n=18446744073709551615UL;return n>0;}",
            1,
        ),
        (
            "floating alias",
            "typedef double real; real f(real x){return x*2;} int main(void){return f(3.5);}",
            7,
        ),
        (
            "binary80 alias",
            "typedef long double real; int main(void){real x=1.0000000000000000001L;return x>1.0L;}",
            1,
        ),
        (
            "pointer alias",
            "typedef int *pointer; int main(void){int n=7;pointer p=&n;return *p;}",
            7,
        ),
        (
            "array alias",
            "typedef int vector[3]; int main(void){vector a;a[1]=7;return a[1]+sizeof(a);}",
            19,
        ),
        (
            "function alias",
            "typedef int fn(int); int f(int x){return x+1;} int main(void){fn *p=f;return p(6);}",
            7,
        ),
        (
            "void alias",
            "typedef void nothing; nothing f(void){return;} int main(void){f();return 7;}",
            7,
        ),
        (
            "multiple declarators",
            "typedef int scalar,*pointer; int main(void){scalar n=7;pointer p=&n;return *p;}",
            7,
        ),
        (
            "alias of alias",
            "typedef long first; typedef first second; int main(void){second n=7;return n;}",
            7,
        ),
        (
            "block alias",
            "int main(void){typedef double real;real x=3.5;return x*2;}",
            7,
        ),
        (
            "inner alias shadows outer",
            "typedef int value; \
            int main(void){\
                value x=1; \
                {\
                    typedef double value;\
                    value y=3.5;\
                    x=y*2;\
                } \
                return x;\
            }",
            7,
        ),
        (
            "object shadows typedef",
            "typedef int value; int main(void){int value=7;return value;}",
            7,
        ),
        (
            "nested alias restored",
            "typedef int value; \
            int main(void){\
                int x=0;\
                {\
                    typedef double value;\
                    value y=3.5;\
                    x=y*2;\
                } \
                {\
                    value z=2;\
                    x+=sizeof(z);\
                } \
                return x;\
            }",
            11,
        ),
        (
            "type and declarator same spelling",
            "typedef int value; int main(void){value value=7;return value;}",
            7,
        ),
        (
            "qualified alias",
            "typedef int value; int main(void){const value n=7;return n;}",
            7,
        ),
        (
            "typedef used in cast",
            "typedef double real; int main(void){return (real)7/2*2;}",
            7,
        ),
        (
            "typedef used in sizeof",
            "typedef long double real; int main(void){return sizeof(real);}",
            16,
        ),
    ] {
        cases.push(Case::new(
            format!("typedef_{:03}",cases.len()+1),
            purpose,
            source,
            Expectation::Exit(expected),
        ));
    }
    for source in [
        "typedef int name; int name; int main(void){return 0;}",
        "int name; typedef int name; int main(void){return 0;}",
        "int main(void){typedef int name; typedef int name;return 0;}",
        "int main(void){int name;typedef int name;return 0;}",
    ] {
        cases.push(Case::new(
            format!("typedef_{:03}", cases.len() + 1),
            source,
            source,
            Expectation::Reject("E009".into()),
        ));
    }
    for (source, code) in [
        ("typedef int name; int main(void){return name;}", "E008"),
        (
            "int main(void){{typedef int name;}name x;return 0;}",
            "E009",
        ),
    ] {
        cases.push(Case::new(
            format!("typedef_{:03}", cases.len() + 1),
            source,
            source,
            Expectation::Reject(code.into()),
        ));
    }
    cases
}
