// Buffering cases for runtime/stdio.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source, input, expected) in [
        (
            "explicit_declaration",
            "runtime binding does not depend on a header being included",
            "int printf(const char *,...);int main(void){return printf(\"%d %.2f\",7,1.25)==6?0:1;}",
            "",
            "7 1.25",
        ),
        (
            "block_declaration",
            "a block function declaration references its canonical runtime entity",
            "int main(void){extern int printf(const char *,...);return printf(\"block\")==5?0:1;}",
            "",
            "block",
        ),
        (
            "local_function_pointers",
            "address-only local references select all three runtime functions",
            "#include <stdio.h>\n\
            int main(void){\
                int (*print)(const char *,...)=printf;\
                int (*scan)(const char *,...)=scanf;\
                int (*next)(void)=getchar;\
                int x;\
                int n=scan(\"%d\",&x);\
                print(\"%d:%d:%d\",n,x,next());\
                return 0;\
            }",
            "42!",
            "1:42:33",
        ),
        (
            "global_function_pointers",
            "global initializer relocations target runtime aliases in both output modes",
            "#include <stdio.h>\n\
            int (*print)(const char *,...)=printf;\
            int (*scan)(const char *,...)=scanf;\
            int (*next)(void)=getchar;\
            int main(void){\
                int x;\
                int n=scan(\"%d\",&x);\
                print(\"%d:%d:%d\",n,x,next());\
                return 0;\
            }",
            "42!",
            "1:42:33",
        ),
        (
            "override_printf",
            "a user printf definition remains authoritative while getchar activates the runtime",
            "#include <stdio.h>\n\
            int printf(const char *format,...){\
                return format[0];\
            }\
            int main(void){\
                return printf(\"A\")+getchar()==131?0:1;\
            }",
            "B",
            "",
        ),
        (
            "override_scanf",
            "a user scanf definition remains authoritative while printf activates the runtime",
            "#include <stdio.h>\n\
            int scanf(const char *format,...){\
                return format[0];\
            }\
            int main(void){\
                int x;\
                printf(\"%d\",scanf(\"A\",&x));\
                return 0;\
            }",
            "",
            "65",
        ),
        (
            "override_getchar",
            "a user getchar definition remains authoritative while printf activates the runtime",
            "#include <stdio.h>\n\
            int getchar(void){\
                return 73;\
            }\
            int main(void){\
                printf(\"%d\",getchar());\
                return 0;\
            }",
            "",
            "73",
        ),
    ] {
        cases.push(Case::new(
            format!("stdio_binding_{id}"),
            purpose,
            source,
            Expectation::Io(input.into(), expected.into(), 0),
        ));
    }
    cases.push(Case::new(
        "stdio_binding_external_module",
        "a module without main leaves stdio function addresses for the external linker",
        "#include <stdio.h>\n\
        int (*print)(const char *,...)=printf;\
        int (*scan)(const char *,...)=scanf;\
        int (*next)(void)=getchar;\
        int probe(void){\
            int x;\
            int n=print(\"ab\",7)+scan(\"xy\",&x)+next();\
            return n+x;\
        }",
        Expectation::AssemblyHarness(
            r#"bits 64
default rel
section .text
global _start,printf,scanf,getchar
extern probe
_start:
call probe
cmp eax,34
setne dil
movzx edi,dil
mov eax,60
syscall
printf:
mov eax,3
ret
scanf:
mov dword [rsi],19
mov eax,5
ret
getchar:
mov eax,7
ret
section .note.GNU-stack noalloc noexec nowrite progbits
"#
            .into(),
        ),
    ));
}
