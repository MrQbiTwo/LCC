// A foreign consumer must see final sizes even when LCC never references a definition.
use crate::{Case, Expectation};

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (name, provider, consumer) in [
        (
            "struct",
            "struct S first,second;struct S{char c;long n;};",
            "struct S{\
                char c;\
                long n;\
            };\
            extern struct S first,second;\
            int driver(void){\
                int zero=first.c==0&&first.n==0&&second.c==0&&second.n==0;\
                first.n=7;\
                second.n=13;\
                return zero&&first.n==7&&second.n==13&&&first!=&second&&(unsigned long)&first%8==0&&\
                (unsigned long)&second%8==0;\
            }",
        ),
        (
            "union",
            "union U first,second;union U{double d;long n;};",
            "union U{\
                double d;\
                long n;\
            };\
            extern union U first,second;\
            int driver(void){\
                int zero=first.d==0.0&&second.d==0.0;\
                first.n=17;\
                second.n=23;\
                return zero&&first.n==17&&second.n==23&&&first!=&second&&(unsigned long)&first%8==0&&\
                (unsigned long)&second%8==0;\
            }",
        ),
        (
            "qualified",
            "struct S;\
            const struct S first;\
            volatile struct S second;\
            const volatile struct S third;\
            struct S{\
                char c;\
                long double n;\
            };",
            "struct S{\
                char c;\
                long double n;\
            };\
            extern const struct S first;\
            extern volatile struct S second;\
            extern const volatile struct S third;\
            int driver(void){\
                int zero=first.c==0&&first.n==0.0L&&second.c==0&&second.n==0.0L&&third.c==0&&third.n==0.0L;\
                second.n=3.25L;\
                return zero&&second.n==3.25L&&first.n==0.0L&&third.n==0.0L&&&first!=&second&&&second!=&third&&\
                &first!=&third&&(unsigned long)&first%16==0&&(unsigned long)&second%16==0&&(unsigned long)&third%16==0;\
            }",
        ),
        (
            "typedef",
            "typedef union U T;typedef const T C;C first;volatile T second;union U{long n;char bytes[24];};",
            "union U{\
                long n;\
                char bytes[24];\
            };\
            extern const union U first;\
            extern volatile union U second;\
            int driver(void){\
                int zero=first.n==0&&second.n==0;\
                second.n=29;\
                return zero&&first.n==0&&second.n==29&&&first!=&second&&(unsigned long)&first%8==0&&\
                (unsigned long)&second%8==0;\
            }",
        ),
    ] {
        for (direction, source, foreign) in [("in", provider, consumer), ("out", consumer, provider)] {
            cases.push(Case::new(
                format!("interop_tentative_{name}_{direction}"),
                format!("foreign late-completed tentative {name} storage {direction}"),
                source,
                Expectation::CInterop(foreign.into()),
            ));
        }
    }
    cases
}
