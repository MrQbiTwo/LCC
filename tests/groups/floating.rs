// Floating expressions exercise representation changes and scalar ABI boundaries.
use crate::{Case, Expectation};

// Each source has an independent expected result and runs through both emitters.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    let mut add = |purpose: String, source: String, expected: i32| {
        cases.push(Case::new(
            format!("floating_{:03}", cases.len() + 1),
            purpose,
            source,
            Expectation::Exit(expected & 255),
        ));
    };
    for (ty, suffix) in [("float", "f"), ("double", ""), ("long double", "L")] {
        for (expr, expected) in [
            ("a+b", 10),
            ("a-b", 5),
            ("a*b", 18),
            ("a/b", 3),
            ("a>b", 1),
            ("a<b", 0),
            ("a==b", 0),
            ("a!=b", 1),
            ("a>=b", 1),
            ("a<=b", 0),
            ("b==b", 1),
            ("b<=b", 1),
            ("-a", -7),
            ("!a", 0),
            ("!!a", 1),
            ("a&&b", 1),
            ("a||b", 1),
            ("a+2", 9),
            ("2+a", 9),
            ("a*2", 15),
        ] {
            add(
                format!("{ty} {expr}"),
                format!("int main(void){{{ty} a=7.5{suffix},b=2.5{suffix}; return {expr};}}"),
                expected,
            );
        }
        for (purpose, body, expected) in [
            (
                "negative zero truth",
                "a=-0.0; if(a) return 7; return !a;",
                1,
            ),
            (
                "compound conversion",
                "a=1.5; a+=2; a*=3; a-=1; a/=2; return a;",
                4,
            ),
            (
                "postfix increment",
                "a=3.5; b=a++; return a*10+b;",
                48,
            ),
            (
                "prefix increment",
                "a=3.5; b=++a; return a*10+b;",
                49,
            ),
            (
                "postfix decrement",
                "a=3.5; b=a--; return a*10+b;",
                28,
            ),
            (
                "prefix decrement",
                "a=3.5; b=--a; return a*10+b;",
                27,
            ),
            (
                "conditional conversion",
                "a=0.0; b=a?2:3.5; return b*2;",
                7,
            ),
            (
                "loop condition",
                "a=3.0; b=0.0; while(a){b+=a; a-=1;} return b;",
                6,
            ),
            (
                "NaN comparisons",
                "a=0.0; b=a/a; return (b==b)*1+(b!=b)*2+(b<b)*4+(b<=b)*8+(b>b)*16+(b>=b)*32+(!b)*64;",
                2,
            ),
            (
                "short circuit",
                "a=0.0; b=1; a&&(b=7); return b;",
                1,
            ),
        ] {
            add(format!("{ty} {purpose}"),format!("int main(void){{{ty} a,b; {body}}}"),expected);
        }
        add(
            format!("{ty} function result"),
            format!(
                "{ty} f({ty} a,int b){{return a+b;}} int main(void){{return f(2.5{suffix},4)*2;}}"
            ),
            13,
        );
        add(
            format!("{ty} nested calls"),
            format!("{ty} f({ty} a){{return a+1;}} int main(void){{return f(f(2.5{suffix}))+f(3.5{suffix});}}"),
            9,
        );
        add(
            format!("{ty} function pointer"),
            format!("{ty} f({ty} x){{return x*2;}} int main(void){{{ty} (*p)({ty})=f; return p(4.5{suffix});}}"),
            9,
        );
        for count in [7, 8, 9, 10] {
            let params = (0..count)
                .map(|i| format!("{ty} a{i}"))
                .collect::<Vec<_>>()
                .join(",");
            let args = (1..=count)
                .map(|i| format!("{i}.0{suffix}"))
                .collect::<Vec<_>>()
                .join(",");
            let sum = (0..count)
                .map(|i| format!("a{i}*{}", i + 1))
                .collect::<Vec<_>>()
                .join("+");
            add(
                format!("{ty} {count} parameters"),
                format!("{ty} f({params}){{return {sum};}} int main(void){{return f({args});}}"),
                (1..=count).map(|i| i * i).sum(),
            );
        }
    }
    for (expr, expected) in [
        ("(float)16777217 == 16777216.0f", 1),
        ("(double)9007199254740993UL == 9007199254740992.0", 1),
        ("(long double)9007199254740993UL > 9007199254740992.0L", 1),
        (
            "(unsigned long)9223372036854777856.0 == 9223372036854777856UL",
            1,
        ),
        (
            "(unsigned long)18446744073709551615.0L == 18446744073709551615UL",
            1,
        ),
        (
            "(double)18446744073709551615UL == 18446744073709551616.0",
            1,
        ),
        (
            "(float)18446744073709551615UL == 18446744073709551616.0f",
            1,
        ),
        (
            "(long double)18446744073709551615UL == 18446744073709551615.0L",
            1,
        ),
        ("(long) -7.9L == -7L", 1),
        ("(double)(float)1.25L == 1.25", 1),
        ("(float)(double)1.25L == 1.25f", 1),
        ("(long double)(float)1.25 == 1.25L", 1),
        ("1.000000059604644775390625f == 1.0f", 1),
        (
            "1.00000000000000011102230246251565404236316680908203125 == 1.0",
            1,
        ),
        ("1.0000000000000000001L > 1.0L", 1),
        ("1e-45f > 0.0f", 1),
        ("5e-324 > 0.0", 1),
        ("1e-5000L == 0.0L", 1),
    ] {
        add(
            expr.into(),
            format!("int main(void){{return {expr};}}"),
            expected,
        );
    }
    add(
        "independent register banks and aligned overflow".into(),
        "double f(int a,double b,int c,float d,int e,double f,int g,double h,int i,double j,\
        int k,double l,int m,long double n,double o,double p,double q,double r,int s){\
            return a+b+c+d+e+f+g+h+i+j+k+l+m+n+o+p+q+r+s;\
        } \
        int main(void){\
            return f(1,2.,3,4.f,5,6.,7,8.,9,10.,11,12.,13,14.L,15.,16.,17.,18.,19);\
        }"
        .into(),
        190,
    );
    cases
}
