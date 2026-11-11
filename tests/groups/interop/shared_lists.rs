// Shared lists cases for interop.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // A helper compiled by the other compiler advances the producer's actual list.
    let producer = "#include <stdarg.h>\n\
    struct S{\
        double x;\
        unsigned a:3;\
        signed int b:5;\
    };\
    int consume(va_list);\
    int target(int n,...){\
        va_list a;\
        int ok,tail;\
        va_start(a,n);\
        ok=consume(a);\
        tail=va_arg(a,int);\
        va_end(a);\
        return ok&&tail==9;\
    }";
    let consumer = "#include <stdarg.h>\n\
    struct S{\
        double x;\
        unsigned a:3;\
        signed int b:5;\
    };\
    int target(int n,...);\
    int consume(va_list a){\
        va_list b;\
        int x,y;\
        struct S s;\
        long double z;\
        x=va_arg(a,int);\
        va_copy(b,a);\
        s=va_arg(b,struct S);\
        y=s.a==5&&s.b==-7&&s.x==3.25;\
        va_end(b);\
        s=va_arg(a,struct S);\
        z=va_arg(a,long double);\
        return x==7&&y&&s.a==5&&s.b==-7&&s.x==3.25&&z==4.5L;\
    }\
    int driver(void){\
        struct S s={\
            3.25,5,-7};\
        return target(0,7,s,4.5L,9);\
    }";
    for (name, source, foreign) in [
        ("list_producer", producer, consumer),
        ("list_consumer", consumer, producer),
    ] {
        cases.push(Case::new(
            format!("interop_{name}"),
            "foreign va_list helper, copy and subsequent original traversal",
            source,
            Expectation::CInterop(foreign.into()),
        ));
    }
}
