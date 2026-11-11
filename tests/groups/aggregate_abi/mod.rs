// Independent ABI probes and declarative case assembly.
use crate::{Case, Expectation};
mod probe;
use probe::*;

pub fn cases() -> Vec<Case> {
    use Class::{Integer as I, Sse as S};
    let chunk = |class, bits, bytes| Chunk { class, bits, bytes };
    let shapes = [
        Shape {
            name: "byte",
            declaration: "typedef struct{char x;} T;",
            init: "{7}",
            check: "s.x==7",
            layout: Layout::Registers(vec![chunk(I, 7, 1)]),
        },
        Shape {
            name: "three_bytes",
            declaration: "typedef struct{char x[3];} T;",
            init: "{{1,2,4}}",
            check: "s.x[0]==1&&s.x[1]==2&&s.x[2]==4",
            layout: Layout::Registers(vec![chunk(I, 0x040201, 3)]),
        },
        Shape {
            name: "integer_pair",
            declaration: "typedef struct{long x,y;} T;",
            init: "{3,4}",
            check: "s.x==3&&s.y==4",
            layout: Layout::Registers(vec![chunk(I, 3, 8), chunk(I, 4, 8)]),
        },
        Shape {
            name: "single_float",
            declaration: "typedef struct{float x;} T;",
            init: "{1.5f}",
            check: "s.x==1.5f",
            layout: Layout::Registers(vec![chunk(S, 0x3fc00000, 4)]),
        },
        Shape {
            name: "two_floats",
            declaration: "typedef struct{float x,y;} T;",
            init: "{1.5f,2.5f}",
            check: "s.x==1.5f&&s.y==2.5f",
            layout: Layout::Registers(vec![chunk(S, 0x402000003fc00000, 8)]),
        },
        Shape {
            name: "three_floats",
            declaration: "typedef struct{float x,y,z;} T;",
            init: "{1.5f,2.5f,4.0f}",
            check: "s.x==1.5f&&s.y==2.5f&&s.z==4.0f",
            layout: Layout::Registers(vec![
                chunk(S, 0x402000003fc00000, 8),
                chunk(S, 0x40800000, 4),
            ]),
        },
        Shape {
            name: "double_pair",
            declaration: "typedef struct{double x,y;} T;",
            init: "{1.5,2.5}",
            check: "s.x==1.5&&s.y==2.5",
            layout: Layout::Registers(vec![
                chunk(S, 0x3ff8000000000000, 8),
                chunk(S, 0x4004000000000000, 8),
            ]),
        },
        Shape {
            name: "integer_sse",
            declaration: "typedef struct{long x;double y;} T;",
            init: "{7,2.5}",
            check: "s.x==7&&s.y==2.5",
            layout: Layout::Registers(vec![chunk(I, 7, 8), chunk(S, 0x4004000000000000, 8)]),
        },
        Shape {
            name: "sse_integer",
            declaration: "typedef struct{double x;long y;} T;",
            init: "{2.5,7}",
            check: "s.x==2.5&&s.y==7",
            layout: Layout::Registers(vec![chunk(S, 0x4004000000000000, 8), chunk(I, 7, 8)]),
        },
        Shape {
            name: "merged_integer",
            declaration: "typedef struct{float x;int y;} T;",
            init: "{1.5f,7}",
            check: "s.x==1.5f&&s.y==7",
            layout: Layout::Registers(vec![chunk(I, 0x000000073fc00000, 8)]),
        },
        Shape {
            name: "union_integer",
            declaration: "typedef union{double x;long y;} T;",
            init: "{2.5}",
            check: "s.x==2.5",
            layout: Layout::Registers(vec![chunk(I, 0x4004000000000000, 8)]),
        },
        Shape {
            name: "nested_sse",
            declaration: "typedef struct{struct N{float x,y;} n;double z;} T;",
            init: "{{1.5f,2.5f},4.0}",
            check: "s.n.x==1.5f&&s.n.y==2.5f&&s.z==4.0",
            layout: Layout::Registers(vec![
                chunk(S, 0x402000003fc00000, 8),
                chunk(S, 0x4010000000000000, 8),
            ]),
        },
        Shape {
            name: "array_integer",
            declaration: "typedef struct{int x[3];} T;",
            init: "{{1,2,4}}",
            check: "s.x[0]==1&&s.x[1]==2&&s.x[2]==4",
            layout: Layout::Registers(vec![chunk(I, 0x0000000200000001, 8), chunk(I, 4, 4)]),
        },
        Shape {
            name: "large_memory",
            declaration: "typedef struct{long x,y,z;} T;",
            init: "{1,2,4}",
            check: "s.x==1&&s.y==2&&s.z==4",
            layout: Layout::Memory(vec![1, 2, 4], 8),
        },
        Shape {
            name: "extended",
            declaration: "typedef struct{long double x;} T;",
            init: "{1.5L}",
            check: "s.x==1.5L",
            layout: Layout::Extended(0xc000000000000000, 0x3fff),
        },
        Shape {
            name: "orphan_x87up",
            declaration: "typedef union{long double x;long y;} T;",
            init: "{1.5L}",
            check: "s.x==1.5L",
            layout: Layout::Memory(vec![0xc000000000000000, 0x3fff], 2),
        },
        Shape {
            name: "union_x87_sse",
            declaration: "typedef union{long double x;double y;} T;",
            init: "{1.5L}",
            check: "s.x==1.5L",
            layout: Layout::Memory(vec![0xc000000000000000, 0x3fff], 2),
        },
        Shape {
            name: "union_integer_over_x87",
            declaration: "typedef union{long double x;long y[2];} T;",
            init: "{1.5L}",
            check: "s.x==1.5L",
            layout: Layout::Registers(vec![chunk(I, 0xc000000000000000, 8), chunk(I, 0x3fff, 2)]),
        },
    ];
    let mut cases = Vec::new();
    for shape in shapes {
        let (put_args, check_args, put_return, check_return) = match &shape.layout {
            Layout::Registers(chunks) => (
                registers(chunks, false, true),
                registers(chunks, false, false),
                registers(chunks, true, true),
                registers(chunks, true, false),
            ),
            Layout::Memory(words, tail) => (
                memory(words, "rsp", 0, true, *tail),
                memory(words, "rsp", 8, false, *tail),
                format!("{}mov rax,rdi\n", memory(words, "rdi", 0, true, *tail)),
                format!(
                    "cmp rax,rsp\njne .bad\n{}cmp qword [rsp+{}],0x55667788\njne .bad\n",
                    memory(words, "rsp", 0, false, *tail),
                    words.len() * 8
                ),
            ),
            Layout::Extended(low, high) => {
                let words = [*low, u64::from(*high)];
                (memory(&words,"rsp",0,true,2),format!("mov rax,[rsp+8]\n{}movzx eax,word [rsp+16]\n{}",compare(*low,8),compare(u64::from(*high),2)),
                    format!("fld tword [rel value]\nret\nsection .rodata\nvalue: dq 0x{low:x},0x{high:x}\nsection .text\n"),
                    format!("fstp tword [rsp]\nmov rax,[rsp]\n{}movzx eax,word [rsp+8]\n{}",compare(*low,8),compare(u64::from(*high),2)))
            }
        };
        let source = format!(
            "{}int target(T s){{return {};}}",
            shape.declaration, shape.check
        );
        let body = format!("{put_args}call target\ncmp eax,1\njne .bad\n");
        cases.push(Case::new(
            format!("aggregate_abi_{}_argument_in", shape.name),
            "independent caller supplies argument classes",
            source,
            Expectation::AssemblyHarness(driver(&body, "")),
        ));
        let source = format!(
            "{}int foreign(T);int target(void){{T s={};return foreign(s);}}",
            shape.declaration, shape.init
        );
        let foreign = format!(
            "global foreign\nforeign:\n{check_args}mov eax,1\nret\n.bad: xor eax,eax\nret\n"
        );
        cases.push(Case::new(
            format!("aggregate_abi_{}_argument_out", shape.name),
            "independent callee inspects argument classes",
            source,
            Expectation::AssemblyHarness(driver("call target\ncmp eax,1\njne .bad\n", &foreign)),
        ));
        let source = format!(
            "{}T target(void){{T s={};return s;}}",
            shape.declaration, shape.init
        );
        let setup = match &shape.layout {
            Layout::Memory(words, _) => format!(
                "lea rdi,[rsp]\nmov qword [rsp+{}],0x55667788\n",
                words.len() * 8
            ),
            _ => String::new(),
        };
        let body = format!("{setup}call target\n{check_return}");
        cases.push(Case::new(
            format!("aggregate_abi_{}_result_out", shape.name),
            "independent caller inspects result classes",
            source,
            Expectation::AssemblyHarness(driver(&body, "")),
        ));
        let source = format!(
            "{}T foreign(void);int target(void){{T s=foreign();return {};}}",
            shape.declaration, shape.check
        );
        let foreign = format!("global foreign\nforeign:\n{put_return}ret\n");
        cases.push(Case::new(
            format!("aggregate_abi_{}_result_in", shape.name),
            "independent callee supplies result classes",
            source,
            Expectation::AssemblyHarness(driver("call target\ncmp eax,1\njne .bad\n", &foreign)),
        ));
    }
    cases
}
