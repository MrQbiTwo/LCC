// Relocation boundaries cases for address initializer.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // A large pool object crosses page alignment beside additional relocations.
    let text = format!("{}Z", "a".repeat(4095));
    cases.push(Case::new(
        format!("address_initializer_{:03}", cases.len() + 1),
        "string pool crosses page boundary beside pointer fixups",
        format!(
            "int x=7;\
            char *p=\"{text}\";\
            int *q=&x;\
            char *r=\"end\"+2;\
            int main(void){{\
                return p[4095]=='Z'&&p[4096]==0&&*q==7&&*r=='d';\
            }}"
        ),
        Expectation::Exit(1),
    ));
}
