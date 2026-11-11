// Aggregate fetch cases for vararg.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, decl, ty, init, check) in [
        (
            "short_record",
            "struct S{char x[3];};",
            "struct S",
            "{{1,2,4}}",
            "s.x[0]==1&&s.x[1]==2&&s.x[2]==4",
        ),
        (
            "ii",
            "struct S{long x,y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "ss",
            "struct S{double x,y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "is",
            "struct S{long x;double y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "si",
            "struct S{double x;long y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        ("float", "struct S{float x;};", "struct S", "{3}", "s.x==3"),
        (
            "floats",
            "struct S{float x,y,z;};",
            "struct S",
            "{1,2,4}",
            "s.x==1&&s.y==2&&s.z==4",
        ),
        (
            "memory",
            "struct S{long x,y,z;};",
            "struct S",
            "{1,2,4}",
            "s.x==1&&s.y==2&&s.z==4",
        ),
        (
            "x87",
            "struct S{long double x;};",
            "struct S",
            "{3.25L}",
            "s.x==3.25L",
        ),
        (
            "union",
            "union S{long x;double y;};",
            "union S",
            "{7}",
            "s.x==7",
        ),
        (
            "union_memory",
            "union S{long double x;double y;};",
            "union S",
            "{3.25L}",
            "s.x==3.25L",
        ),
    ] {
        let source = format!(
            "#include <stdarg.h>\n\
            {decl}int f(int n,...){{\
                va_list a;\
                {ty} s;\
                int tail;\
                va_start(a,n);\
                s=va_arg(a,{ty});\
                tail=va_arg(a,int);\
                return ({check})&&tail==9;\
            }}\
            int main(void){{\
                {ty} s={init};\
                return f(1,s,9);\
            }}"
        );
        cases.push(Case::new(
            format!("vararg_aggregate_{name}"),
            format!("aggregate fetch {name}"),
            source,
            Expectation::Exit(1),
        ));
    }
}
