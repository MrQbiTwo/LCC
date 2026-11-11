// Initial executable cases target distinct parser, ABI, and code-selection risks.
use crate::{Case, Expectation};

// Register a source case with an independently specified exit value.
fn add(cases: &mut Vec<Case>, group: &str, purpose: &str, source: String, expected: i32) {
    let number = cases
        .iter()
        .filter(|case| case.id.starts_with(group))
        .count()
        + 1;
    cases.push(Case::new(
        format!("{group}_{number:03}"),
        purpose,
        source,
        Expectation::Exit(expected & 255),
    ));
}

// Build a deterministic catalog; output modes are repetitions of the same case.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (expression, expected) in [
        ("42", 42),
        ("0", 0),
        ("255", 255),
        ("-1", 255),
        ("0xff", 255),
        ("077", 63),
        ("2+3*4", 14),
        ("(2+3)*4", 20),
        ("80/7", 11),
        ("80%7", 3),
        ("-20/3", 250),
        ("-20%3", 254),
        ("1<<5", 32),
        ("128>>3", 16),
        ("(-8)>>2", 254),
        ("7&3", 3),
        ("7|8", 15),
        ("7^3", 4),
        ("~0", 255),
        ("!0", 1),
        ("!7", 0),
        ("5==5", 1),
        ("5!=5", 0),
        ("-3<2", 1),
        ("-3>=2", 0),
        ("7<=7", 1),
        ("8>7", 1),
        ("0&&1", 0),
        ("1||0", 1),
        ("7&&8", 1),
        ("0||0", 0),
        ("1?9:4", 9),
        ("0?9:4", 4),
        ("1,2,3", 3),
        ("sizeof(int)", 4),
        ("sizeof(long)", 8),
        ("sizeof(char*)", 8),
        ("sizeof(int[3])", 12),
        ("(unsigned int)-1 > 0", 1),
        ("(signed char)255", 255),
        ("(unsigned char)257", 1),
        ("(short)65535", 255),
        ("4294967295u+1u == 0", 1),
        ("0xffffffff > 0", 1),
    ] {
        add(
            &mut cases,
            "expr",
            expression,
            format!("int main(void){{return {expression};}}"),
            expected,
        );
    }
    for (purpose, body, expected) in [
        ("initialized local", "int x=13; return x;", 13),
        ("separate declarators", "int x=4,y=7; return x*y;", 28),
        ("chained assignment", "int x,y; x=y=12; return x+y;", 24),
        (
            "compound arithmetic",
            "int x=10; x+=5; x*=3; x-=2; return x;",
            43,
        ),
        ("postfix result", "int x=3,y; y=x++; return 10*y+x;", 34),
        ("prefix result", "int x=3,y; y=++x; return 10*y+x;", 44),
        ("postfix decrement", "int x=5,y; y=x--; return 10*y+x;", 54),
        (
            "address and indirection",
            "int x=7; int *p=&x; *p=18; return x;",
            18,
        ),
        (
            "array subscripts",
            "int a[3]; a[0]=4; a[1]=8; a[2]=9; return a[0]+a[1]+a[2];",
            21,
        ),
        (
            "nested scope shadow",
            "int x=4; {int x=7; x++;} return x;",
            4,
        ),
        ("short circuit and", "int x=0; 0 && x++; return x;", 0),
        ("short circuit or", "int x=0; 1 || x++; return x;", 0),
        (
            "conditional side effect",
            "int x=0; 1 ? x++ : (x=99); return x;",
            1,
        ),
        ("if else", "int x=7; if(x<5) return 8; else return 19;", 19),
        (
            "while sum",
            "int i=0,s=0; while(i<10){s+=i; i++;} return s;",
            45,
        ),
        (
            "for continue",
            "int i,s=0; for(i=0;i<10;i++){if(i==4)continue;s+=i;} return s;",
            41,
        ),
        (
            "for break",
            "int i,s=0; for(i=0;i<10;i++){if(i==4)break;s+=i;} return s;",
            6,
        ),
        (
            "do executes once",
            "int x=0; do{x++;}while(0); return x;",
            1,
        ),
        (
            "nested loop destinations",
            "int i,j,s=0;for(i=0;i<3;i++){for(j=0;j<4;j++){if(j==2)break;s++;}}return s;",
            6,
        ),
        ("void pointer width", "void *p; return sizeof(p);", 8),
    ] {
        add(
            &mut cases,
            "flow",
            purpose,
            format!("int main(void){{{body}}}"),
            expected,
        );
    }
    for count in 0..=10 {
        let parameters = if count == 0 {
            "void".into()
        } else {
            (0..count)
                .map(|i| format!("int p{i}"))
                .collect::<Vec<_>>()
                .join(",")
        };
        let arguments = (0..count)
            .map(|i| (i + 1).to_string())
            .collect::<Vec<_>>()
            .join(",");
        let expression = if count == 0 {
            "17".into()
        } else {
            (0..count)
                .map(|i| format!("{}*p{i}", i + 1))
                .collect::<Vec<_>>()
                .join("+")
        };
        let expected = if count == 0 {
            17
        } else {
            (1..=count).map(|i| i * i).sum()
        };
        add(
            &mut cases,
            "abi",
            "distinguishable register and stack arguments",
            format!("int f({parameters}){{return {expression};}} int main(void){{return f({arguments});}}"),
            expected,
        );
    }
    for (purpose, source, expected) in [
        (
            "recursive calls",
            "int f(int n){if(n<2)return 1;return n*f(n-1);}int main(void){return f(5);}",
            120,
        ),
        (
            "forward prototype",
            "int f(int x);int main(void){return f(12);}int f(int x){return x+3;}",
            15,
        ),
        (
            "function pointer call",
            "int f(int x){return x*3;}int main(void){int (*p)(int)=f;return p(7);}",
            21,
        ),
        (
            "global initializer",
            "int g=17;int main(void){g+=4;return g;}",
            21,
        ),
        (
            "zero global array",
            "int a[4];int main(void){a[2]=13;return a[0]+a[2]+a[3];}",
            13,
        ),
        (
            "narrow return",
            "signed char f(void){return 255;}int main(void){return f()<0;}",
            1,
        ),
        (
            "narrow parameter",
            "int f(unsigned char x){return x;}int main(void){return f(257);}",
            1,
        ),
    ] {
        add(&mut cases, "parse", purpose, source.into(), expected);
    }
    for (source, code, purpose) in [
        ("int main(void){return x;}", "E009", "undeclared object"),
        ("int main(void){return 09;}", "E007", "invalid octal digit"),
        (
            "int main(void){return 0x;}",
            "E007",
            "missing hexadecimal digits",
        ),
        ("int main(void){return 1}", "E006", "missing semicolon"),
        ("int main(void){break;}", "E006", "break outside loop"),
        ("int main(void){continue;}", "E006", "continue outside loop"),
        (
            "int main(void){int a;int a;return 0;}",
            "E009",
            "duplicate local",
        ),
        (
            "int main(void){return 18446744073709551616u;}",
            "E011",
            "integer literal overflow",
        ),
        (
            "int main(void){/* unterminated",
            "E007",
            "unterminated block comment",
        ),
        ("int f(void){return 1;}", "E010", "missing entry point"),
        ("int main(void){return @;}", "E007", "invalid source byte"),
        (
            "int main(void){1=2;return 0;}",
            "E008",
            "non-lvalue assignment",
        ),
    ] {
        let number = cases
            .iter()
            .filter(|case| case.id.starts_with("errors_"))
            .count()
            + 1;
        cases.push(Case::new(
            format!("errors_{number:03}"),
            purpose,
            source,
            Expectation::Reject(code.into()),
        ));
    }
    cases
}
