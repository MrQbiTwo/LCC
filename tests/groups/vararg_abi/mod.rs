// Independent ABI probes and declarative case assembly.
use crate::Case;
mod probe;
use probe::*;

pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (name, decl, ty, check, setup) in [
        (
            "integer",
            "",
            "long",
            "s==-7",
            "mov rsi,-7\nxor eax,eax\n",
        ),
        (
            "double",
            "",
            "double",
            "s==3.25",
            "mov rax,0x400a000000000000\nmovq xmm0,rax\nmov eax,1\n",
        ),
        (
            "binary80",
            "",
            "long double",
            "s==3.25L",
            "mov rax,0xd000000000000000\nmov [rsp],rax\nmov qword [rsp+8],0x4000\nxor eax,eax\n",
        ),
        (
            "short_record",
            "struct S{char x[3];};",
            "struct S",
            "s.x[0]==1&&s.x[1]==2&&s.x[2]==4",
            "mov esi,0x040201\nxor eax,eax\n",
        ),
        (
            "ii",
            "struct S{long x,y;};",
            "struct S",
            "s.x==3&&s.y==4",
            "mov esi,3\nmov edx,4\nxor eax,eax\n",
        ),
        (
            "ss",
            "struct S{double x,y;};",
            "struct S",
            "s.x==3&&s.y==4",
            "mov rax,0x4008000000000000\nmovq xmm0,rax\nmov rax,0x4010000000000000\nmovq xmm1,rax\nmov eax,2\n",
        ),
        (
            "is",
            "struct S{long x;double y;};",
            "struct S",
            "s.x==3&&s.y==4",
            "mov esi,3\nmov rax,0x4010000000000000\nmovq xmm0,rax\nmov eax,1\n",
        ),
        (
            "si",
            "struct S{double x;long y;};",
            "struct S",
            "s.x==3&&s.y==4",
            "mov rax,0x4008000000000000\nmovq xmm0,rax\nmov esi,4\nmov eax,1\n",
        ),
        (
            "float_record",
            "struct S{float x;};",
            "struct S",
            "s.x==3.25f",
            "mov eax,0x40500000\nmovd xmm0,eax\nmov eax,1\n",
        ),
        (
            "floats_record",
            "struct S{float x,y,z;};",
            "struct S",
            "s.x==1&&s.y==2&&s.z==4",
            "mov rax,0x400000003f800000\nmovq xmm0,rax\nmov eax,0x40800000\nmovd xmm1,eax\nmov eax,2\n",
        ),
        (
            "memory",
            "struct S{long x,y,z;};",
            "struct S",
            "s.x==1&&s.y==2&&s.z==4",
            "mov qword [rsp],1\nmov qword [rsp+8],2\nmov qword [rsp+16],4\nxor eax,eax\n",
        ),
        (
            "x87_record",
            "struct S{long double x;};",
            "struct S",
            "s.x==3.25L",
            "mov rax,0xd000000000000000\nmov [rsp],rax\nmov qword [rsp+8],0x4000\nxor eax,eax\n",
        ),
        (
            "union",
            "union S{long x;double y;};",
            "union S",
            "s.x==7",
            "mov esi,7\nxor eax,eax\n",
        ),
    ] {
        add(
            &mut cases,
            name,
            format!("{decl}int target(int n,...){{\
                va_list a;\
                {ty} s;\
                va_start(a,n);\
                s=va_arg(a,{ty});\
                va_end(a);\
                return {check};\
            }}"),
            &format!("xor edi,edi\n{setup}call target\ncmp eax,1\njne bad\n"),
            "",
        );
    }
    // The list below is owned and initialized by NASM, including cursor values.
    let base = r#"lea rdi,[rsp+256]
mov dword [rdi],40
mov dword [rdi+4],160
lea rax,[rsp]
mov [rdi+8],rax
lea rax,[rsp+64]
mov [rdi+16],rax
"#;
    add(
        &mut cases,
        "foreign_list_gp",
        "int target(va_list a){long x,y;x=va_arg(a,long);y=va_arg(a,long);return x==7&&y==9;}"
            .into(),
        &format!(
            r#"{base}mov qword [rsp+104],7
mov qword [rsp],9
call target
cmp eax,1
jne bad
cmp dword [rsp+256],48
jne bad
cmp dword [rsp+260],160
jne bad
lea rax,[rsp+8]
cmp [rsp+264],rax
jne bad
"#
        ),
        "",
    );
    add(
        &mut cases,
        "foreign_list_sse",
        "int target(va_list a){double x,y;x=va_arg(a,double);y=va_arg(a,double);return x==3&&y==4;}".into(),
        &format!(r#"{base}mov rax,0x4008000000000000
mov [rsp+224],rax
mov rax,0x4010000000000000
mov [rsp],rax
call target
cmp eax,1
jne bad
cmp dword [rsp+256],40
jne bad
cmp dword [rsp+260],176
jne bad
lea rax,[rsp+8]
cmp [rsp+264],rax
jne bad
"#),
        "",
    );
    add(
        &mut cases,
        "foreign_list_rollback",
        "struct S{\
            long x,y;\
        };\
        int target(va_list a){\
            struct S s;\
            long z;\
            s=va_arg(a,struct S);\
            z=va_arg(a,long);\
            return s.x==3&&s.y==4&&z==7;\
        }"
        .into(),
        &format!(
            r#"{base}mov qword [rsp],3
mov qword [rsp+8],4
mov qword [rsp+104],7
call target
cmp eax,1
jne bad
cmp dword [rsp+256],48
jne bad
lea rax,[rsp+16]
cmp [rsp+264],rax
jne bad
"#
        ),
        "",
    );
    add(
        &mut cases,
        "foreign_list_alignment",
        "int target(va_list a){long double x;x=va_arg(a,long double);return x==3.25L;}".into(),
        &format!(
            r#"{base}lea rax,[rsp+8]
mov [rdi+8],rax
mov rax,0xd000000000000000
mov [rsp+16],rax
mov word [rsp+24],0x4000
call target
cmp eax,1
jne bad
cmp dword [rsp+256],40
jne bad
cmp dword [rsp+260],160
jne bad
lea rax,[rsp+32]
cmp [rsp+264],rax
jne bad
"#
        ),
        "",
    );
    // Verify every saved byte of all eight vector slots, including their upper halves.
    let mut setup =
        "mov edi,0\nmov esi,1\nmov edx,2\nmov ecx,3\nmov r8d,4\nmov r9d,5\n".to_string();
    for i in 0..8 {
        setup += &format!(
            "mov qword [rsp+64],{}\nmov qword [rsp+72],{}\nmovdqu xmm{i},[rsp+64]\n",
            10 + i,
            30 + i
        );
    }
    setup += "mov eax,8\ncall target\ncmp eax,1\njne bad\n";
    let mut foreign = r#"global inspect
inspect:
cmp dword [rdi],8
jne bad
cmp dword [rdi+4],48
jne bad
mov r10,[rdi+16]
"#
    .to_string();
    for i in 0..6 {
        foreign += &format!("cmp qword [r10+{}],{i}\njne bad\n", i * 8);
    }
    for i in 0..8 {
        foreign += &format!(
            "cmp qword [r10+{}],{}\njne bad\ncmp qword [r10+{}],{}\njne bad\n",
            48 + i * 16,
            10 + i,
            56 + i * 16,
            30 + i
        );
    }
    foreign += "mov eax,1\nret\n";
    add(
        &mut cases,
        "full_register_save",
        "int inspect(va_list);int target(int n,...){va_list a;va_start(a,n);return inspect(a);}"
            .into(),
        &setup,
        &foreign,
    );
    // A foreign helper consumes a list created by LCC and advances both cursors.
    add(
        &mut cases,
        "foreign_consumer",
        "int consume(va_list);\
        int target(int n,...){\
            va_list a;\
            int ok;\
            va_start(a,n);\
            ok=consume(a);\
            return ok&&va_arg(a,int)==9;\
        }"
        .into(),
        r#"mov edi,0
mov esi,7
mov edx,9
mov rax,0x4008000000000000
movq xmm0,rax
mov eax,1
call target
cmp eax,1
jne bad
"#,
        r#"global consume
consume:
cmp dword [rdi],8
jne bad
cmp dword [rdi+4],48
jne bad
mov r10,[rdi+16]
cmp qword [r10+8],7
jne bad
mov rax,0x4008000000000000
cmp [r10+48],rax
jne bad
add dword [rdi],8
add dword [rdi+4],16
mov eax,1
ret
"#,
    );
    // Upper-bound AL is permitted, while unused vector registers need no save.
    for count in [0, 8] {
        add(
            &mut cases,
            &format!("al_bound_{count}"),
            "int target(int n,...){va_list a;va_start(a,n);return va_arg(a,int)==7;}".into(),
            &format!("mov edi,0\nmov esi,7\nmov eax,{count}\ncall target\ncmp eax,1\njne bad\n"),
            "",
        );
    }
    cases
}
