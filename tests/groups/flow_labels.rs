// Switch dispatch and function labels exercise distinct nested control-flow paths.
use crate::{Case, Expectation};

// Cases run as native ELF and as independently assembled NASM output.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (purpose, body, expected) in [
        (
            "matched case",
            "int x=2,y=0; switch(x){case 1:y=7;break;case 2:y=9;break;default:y=11;} return y;",
            9,
        ),
        (
            "default selection",
            "int y=0; switch(7){case 1:y=7;break;default:y=11;} return y;",
            11,
        ),
        (
            "unmatched without default",
            "int y=3; switch(7){case 1:y=7;} return y;",
            3,
        ),
        (
            "fall through",
            "int y=0; switch(1){case 1:y+=2;case 2:y+=3;case 3:y+=4;} return y;",
            9,
        ),
        (
            "grouped cases",
            "int y=0; switch(2){case 1:case 2:case 3:y=8;break;} return y;",
            8,
        ),
        (
            "default before case",
            "int y=0; switch(5){default:y=2;case 1:y+=3;} return y;",
            5,
        ),
        (
            "case after default dispatch",
            "int y=0; switch(1){default:y=2;case 1:y+=3;} return y;",
            3,
        ),
        (
            "negative case",
            "switch(-3){case -3:return 7;default:return 9;}",
            7,
        ),
        (
            "unsigned conversion",
            "unsigned int x=4294967295U; switch(x){case -1:return 7;default:return 9;}",
            7,
        ),
        (
            "wide control",
            "long x=4294967297L; switch(x){case 1:return 7;case 4294967297L:return 9;} return 11;",
            9,
        ),
        (
            "integer promotion",
            "unsigned char x=255; switch(x){case -1:return 7;case 255:return 9;} return 11;",
            9,
        ),
        (
            "single evaluation",
            "int x=1,y=0; switch(x++){case 1:y=3;break;} return x*10+y;",
            23,
        ),
        (
            "constant case",
            "switch(9){case 2*4+1:return 7;} return 9;",
            7,
        ),
        (
            "nested switch break",
            "int y=0; \
            switch(1){\
                case 1:switch(2){\
                    case 2:y=3;\
                    break;\
                    default:y=8;\
                } \
                y+=4;\
                break;\
                default:y=9;\
            } \
            return y;",
            7,
        ),
        (
            "continue targets enclosing loop",
            "int i,s=0; for(i=0;i<5;i++){switch(i){case 2:continue;default:s+=i;} s+=10;} return s;",
            48,
        ),
        (
            "loop break restores switch break",
            "int i=0; switch(1){case 1:while(1){i++;break;} i+=2;break;default:i=9;} return i;",
            3,
        ),
        (
            "switch break restores loop break",
            "int i=0; while(1){switch(1){case 1:i++;break;} i+=2;break;} return i;",
            3,
        ),
        (
            "empty switch skips body",
            "int x=0; switch(1){x=7;} return x;",
            0,
        ),
        (
            "case in nested block",
            "int x=0; switch(2){{case 1:x=1;break;case 2:x=2;}} return x;",
            2,
        ),
        (
            "forward goto",
            "int x=1; goto done; x=7; done:return x;",
            1,
        ),
        (
            "backward goto",
            "int x=0; again:x++; if(x<5)goto again; return x;",
            5,
        ),
        (
            "label and object namespaces",
            "int done=7; goto done; done:return done;",
            7,
        ),
        (
            "goto from nested blocks",
            "int x=0; {x=3;{goto done;}} x=7; done:return x;",
            3,
        ),
        (
            "goto into block",
            "int x=0; goto inside; {inside:x=7;} return x;",
            7,
        ),
        (
            "goto exits loop",
            "int x=0; while(1){x++; if(x==3)goto done;} done:return x;",
            3,
        ),
        (
            "goto into switch",
            "int x=0; goto label; switch(0){case 0:x=2;label:x+=3;break;} return x;",
            3,
        ),
        (
            "label before empty statement",
            "int x=1; goto done; x=9; done:; return x;",
            1,
        ),
    ] {
        cases.push(Case::new(
            format!("flow_labels_{:03}",cases.len()+1),
            purpose,
            format!("int main(void){{{body}}}"),
            Expectation::Exit(expected),
        ));
    }
    cases.push(Case::new(
        format!("flow_labels_{:03}", cases.len() + 1),
        "label namespace resets between functions",
        "int f(void){goto x;x:return 2;} int main(void){goto x;x:return f()+3;}",
        Expectation::Exit(5),
    ));
    for (body, code) in [
        ("switch(1){case 1:break;case 1:break;} return 0;", "E009"),
        (
            "switch(1U){case -1:break;case 4294967295U:break;} return 0;",
            "E009",
        ),
        ("switch(1){default:break;default:break;} return 0;", "E009"),
        ("case 1:return 0;", "E006"),
        ("default:return 0;", "E006"),
        ("switch(1.0){default:return 0;}", "E008"),
        ("switch(1){case 1.0:return 0;}", "E008"),
        ("switch(1){case 1/0:return 0;}", "E011"),
        ("switch(1){case 1:continue;} return 0;", "E006"),
        ("goto missing; return 0;", "E009"),
        ("x:;x:return 0;", "E009"),
    ] {
        cases.push(Case::new(
            format!("flow_labels_{:03}", cases.len() + 1),
            body,
            format!("int main(void){{{body}}}"),
            Expectation::Reject(code.into()),
        ));
    }
    for (purpose, body) in [
        (
            "bounded unary recursion",
            format!("return {}1;", "!".repeat(300)),
        ),
        (
            "bounded block recursion",
            format!("{}return 1;{}", "{".repeat(300), "}".repeat(300)),
        ),
    ] {
        cases.push(Case::new(
            format!("flow_labels_{:03}", cases.len() + 1),
            purpose,
            format!("int main(void){{{body}}}"),
            Expectation::Reject("E006".into()),
        ));
    }
    cases
}
