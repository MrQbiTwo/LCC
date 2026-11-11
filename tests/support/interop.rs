// Optional foreign compilers are independent ABI endpoints, never LCC backends.
use crate::process;
use std::{fs, path::Path, time::Duration};

// Bound each build/run and include the failing executable in diagnostics.
fn run(program: &Path, args: &[String]) -> Result<(), String> {
    let result = process::invoke(program, args, Vec::new(), Duration::from_secs(10))?;
    if result.timed_out || result.truncated || result.status != Some(0) {
        return Err(format!("{} {args:?}: {result:?}", program.display()));
    }
    Ok(())
}

// Link one LCC object against GCC/Clang endpoints at two optimization settings.
pub fn execute(compiler: &Path, source: &Path, foreign: &str, dir: &Path) -> Result<(), String> {
    let assembly = dir.join("lcc.asm");
    let object = dir.join("lcc.o");
    let foreign_source = dir.join("foreign.c");
    let foreign_object = dir.join("foreign.o");
    let startup = dir.join("start.asm");
    let startup_object = dir.join("start.o");
    let executable = dir.join("interop.elf");
    let arg = |path: &Path| path.to_string_lossy().into_owned();
    fs::write(&foreign_source, foreign).map_err(|e| e.to_string())?;
    fs::write(
        &startup,
        r#"bits 64
default rel
section .text
global _start
extern driver
_start:
cld
sub rsp,32
mov word [rsp],0x077f
fldcw [rsp]
mov dword [rsp+4],0x3f80
ldmxcsr [rsp+4]
call driver
cmp eax,1
jne bad
fnstcw [rsp+8]
cmp word [rsp+8],0x077f
jne bad
stmxcsr [rsp+12]
mov eax,[rsp+12]
and eax,0xffc0
cmp eax,0x3f80
jne bad
fnstsw ax
test ax,0x3800
jnz bad
xor edi,edi
jmp finish
bad: mov edi,1
finish: mov eax,60
syscall
section .note.GNU-stack noalloc noexec nowrite progbits
"#,
    )
    .map_err(|e| e.to_string())?;
    run(
        compiler,
        &["-S".into(), arg(source), "-o".into(), arg(&assembly)],
    )?;
    for (input, output) in [(&assembly, &object), (&startup, &startup_object)] {
        run(
            Path::new("nasm"),
            &[
                "-f".into(),
                "elf64".into(),
                arg(input),
                "-o".into(),
                arg(output),
            ],
        )?;
    }
    for cc in ["gcc", "clang"] {
        for optimization in ["-O0", "-O2"] {
            run(
                Path::new(cc),
                &[
                    optimization.into(),
                    "-std=gnu89".into(),
                    "-ffreestanding".into(),
                    "-fno-builtin".into(),
                    "-fno-pie".into(),
                    "-fno-stack-protector".into(),
                    "-c".into(),
                    arg(&foreign_source),
                    "-o".into(),
                    arg(&foreign_object),
                ],
            )?;
            run(
                Path::new("ld"),
                &[
                    "-z".into(),
                    "noexecstack".into(),
                    arg(&object),
                    arg(&foreign_object),
                    arg(&startup_object),
                    "-o".into(),
                    arg(&executable),
                ],
            )?;
            run(&executable, &[]).map_err(|e| format!("{cc} {optimization}: {e}"))?;
        }
    }
    Ok(())
}
