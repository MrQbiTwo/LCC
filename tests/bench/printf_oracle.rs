// Compare the original printf formatter against independent libc-backed callers.
use std::{env, fs, path::Path, process::Command};

// Commands preserve complete diagnostics when an assembler, compiler, or probe fails.
fn run(program: &str, args: &[&str]) -> Vec<u8> {
    let output = Command::new(program)
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("cannot run {program}: {e}"));
    assert!(
        output.status.success(),
        "{program} {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    output.stdout
}

// The isolated sink exercises formatting without depending on input buffering.
const ASSEMBLY: &str = r#"bits 64
default rel
%include "runtime/linux_x86_64/decimal.asm"
%include "runtime/linux_x86_64/stdio/output.asm"
section .text
lcc_rt_write:
    mov rdx, rsi
    mov rsi, rdi
    mov edi, 1
    mov eax, 1
    syscall
    ret
section .note.GNU-stack noalloc noexec nowrite progbits
"#;

// Emit deterministic finite values with difficult binary/decimal boundaries.
fn source() -> String {
    let mut source = String::from(
        r#"#include <stdio.h>
#ifdef LCC
#define printf lcc_rt_printf
extern int lcc_rt_printf(const char *, ...);
#endif
int main(void) {
union { unsigned long u; double d; } n;
int r;
r=printf("literal %% [%d] [%i] [%u] [%ld] [%lu] [%hd] [%hu] [%c] [%s]\n", -2147483647-1,42,4294967295u,-9223372036854775807L-1,18446744073709551615UL,-32768,65535,'Q',"hello");
printf("count=%d\n",r);
printf("[%+08d] [% 8.5d] [%-08d] [%.0d] [%8.0d] [%*.*s]\n",-13,42,13,0,0,-10,3,"abcdef");
printf("[%f] [%.2lf] [%.0f] [%.0f] [%.0f] [%+010.2f]\n",1.25,2.675,0.5,1.5,2.5,-0.0);
printf("%d %f %d %f %d %f %d %f %d %f %d %f %d %f %d %f %d %f %d %f\n",1,1.,2,2.,3,3.,4,4.,5,5.,6,6.,7,7.,8,8.,9,9.,10,10.);
printf("[%*.*f] [%*.*f] [%*.*d] [%.*s]\n",-20,4,-1.25,20,-1,1.25,12,8,-123,-1,"abcdef");
printf("[%4096d] [%.4096d] [%4096s] [%4096c]\n",1,-1,"x",'y');
"#,
    );
    let edges: [u64; 24] = [
        0,
        1,
        2,
        0x000f_ffff_ffff_ffff,
        0x0010_0000_0000_0000,
        0x0010_0000_0000_0001,
        0x3fdfffffffffffff,
        0x3fe0000000000000,
        0x3fe0000000000001,
        0x3ff0000000000000,
        0x3ff8000000000000,
        0x4004000000000000,
        0x4005666666666666,
        0x4340000000000000,
        0x7fefffffffffffff,
        0x8000000000000000,
        0x8000000000000001,
        0x8010000000000000,
        0xbff8000000000000,
        0xffefffffffffffff,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000000,
        0xfff8000000000000,
    ];
    let precisions = [0, 1, 2, 6, 17, 53, 324, 1074];
    for bits in edges {
        for precision in precisions.into_iter().chain([4096]) {
            source.push_str(&format!("n.u=0x{bits:016x}UL;r=printf(\"[%+0*.*f]\\n\",40,{precision},n.d);printf(\"count=%d\\n\",r);\n"));
        }
    }
    let mut seed = 0x1a9845b572eca331u64;
    for _ in 0..256 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let bits = if seed & 0x7ff0000000000000 == 0x7ff0000000000000 {
            seed ^ 0x0010000000000000
        } else {
            seed
        };
        for precision in precisions {
            source.push_str(&format!(
                "n.u=0x{bits:016x}UL;printf(\"[%.*f]\\n\",{precision},n.d);\n"
            ));
        }
    }
    source.push_str("return 0;\n}\n");
    source
}

// Run from the repository root; retain fixtures and outputs for reproducibility.
fn main() {
    let destination = env::args()
        .nth(1)
        .unwrap_or_else(|| "build/printf_oracle".into());
    let directory = Path::new(&destination);
    fs::create_dir_all(directory).unwrap();
    let asm = directory.join("runtime.asm");
    let object = directory.join("runtime.o");
    let c_source = directory.join("probe.c");
    fs::write(&asm, ASSEMBLY).unwrap();
    fs::write(&c_source, source()).unwrap();
    run(
        "nasm",
        &[
            "-f",
            "elf64",
            "-I./",
            asm.to_str().unwrap(),
            "-o",
            object.to_str().unwrap(),
        ],
    );
    let mut expected = None;
    for compiler in ["gcc", "clang"] {
        for optimization in ["-O0", "-O2"] {
            for runtime in [false, true] {
                let name = format!(
                    "{compiler}_{}_{}",
                    &optimization[1..],
                    if runtime { "lcc" } else { "reference" }
                );
                let executable = directory.join(&name);
                let mut args = vec![
                    optimization,
                    "-fno-builtin",
                    c_source.to_str().unwrap(),
                    "-o",
                    executable.to_str().unwrap(),
                ];
                if runtime {
                    args.extend(["-DLCC", object.to_str().unwrap()]);
                }
                run(compiler, &args);
                let output = run(executable.to_str().unwrap(), &[]);
                fs::write(directory.join(format!("{name}.txt")), &output).unwrap();
                if let Some(reference) = &expected {
                    assert!(
                        reference == &output,
                        "output mismatch in {name}; retained output in {destination}"
                    );
                } else {
                    expected = Some(output);
                }
            }
        }
    }
    println!("printf oracle passed: 2264 floating conversions plus field/vararg cases, GCC and Clang at O0/O2");
}
