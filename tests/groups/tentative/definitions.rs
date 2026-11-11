// Definitions cases for tentative.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "struct_zero",
            "a late-completed tentative struct is zero initialized",
            "struct S object;struct S{int n;long m;};int main(void){return object.n!=0||object.m!=0;}",
        ),
        (
            "union_zero",
            "a late-completed tentative union initializes its first member",
            "union U object;union U{double d;long n;};int main(void){return object.d!=0.0;}",
        ),
        (
            "struct_write",
            "late struct layout supports member writes",
            "struct S object;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                object.c=3;\
                object.n=19;\
                return !(sizeof(object)==16&&object.c==3&&object.n==19);\
            }",
        ),
        (
            "union_write",
            "late union layout has the complete size and alignment",
            "union U object;\
            union U{\
                char c;\
                long n;\
            };\
            int main(void){\
                object.n=23;\
                return !(sizeof(object)==8&&object.n==23&&(unsigned long)&object%8==0);\
            }",
        ),
        (
            "typedef",
            "an incomplete record typedef follows its tag",
            "typedef struct S T;\
            T object;\
            struct S{\
                int n;\
            };\
            int main(void){\
                object.n=17;\
                return object.n!=17;\
            }",
        ),
        (
            "const",
            "a const forward copy gets complete storage",
            "struct S;\
            const struct S object;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                return !(sizeof(object)==16&&object.c==0&&object.n==0);\
            }",
        ),
        (
            "volatile",
            "a volatile forward copy gets complete storage",
            "struct S;\
            volatile struct S object;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                object.n=31;\
                return !(sizeof(object)==16&&object.c==0&&object.n==31);\
            }",
        ),
        (
            "const_volatile",
            "combined qualification resolves late record layout",
            "union U;\
            const volatile union U object;\
            union U{\
                long n;\
                double d;\
            };\
            int main(void){\
                return !(sizeof(object)==8&&object.n==0);\
            }",
        ),
        (
            "qualified_typedef",
            "qualified aliases retain the canonical forward tag",
            "typedef struct S T;\
            typedef const T C;\
            C object;\
            struct S{\
                long a;\
                long b;\
            };\
            int main(void){\
                return !(sizeof(object)==16&&object.a==0&&object.b==0);\
            }",
        ),
        (
            "typedef_volatile",
            "adding volatile to an incomplete typedef preserves completion",
            "typedef union U T;\
            volatile T object;\
            union U{\
                long n;\
            };\
            int main(void){\
                object.n=37;\
                return object.n!=37;\
            }",
        ),
        (
            "multiple",
            "multiple names receive distinct complete storage",
            "struct S a,b,c;\
            struct S{\
                char c;\
                long n;\
            };\
            int main(void){\
                a.n=3;\
                b.n=5;\
                c.n=7;\
                return !(a.n==3&&b.n==5&&c.n==7&&a.c==0&&b.c==0&&c.c==0&&&a!=&b&&&b!=&c);\
            }",
        ),
        (
            "multiple_qualifiers",
            "separately qualified objects do not share storage",
            "struct S;\
            struct S a;\
            volatile struct S b;\
            const struct S c;\
            struct S{\
                long n;\
            };\
            int main(void){\
                a.n=7;\
                b.n=11;\
                return !(a.n==7&&b.n==11&&c.n==0&&&a!=&b&&&a!=&c&&&b!=&c);\
            }",
        ),
        (
            "repeated",
            "repeated tentative declarations denote one object",
            "struct S object;\
            struct S object;\
            struct S{\
                int n;\
            };\
            struct S object;\
            int main(void){\
                object.n=41;\
                return object.n!=41;\
            }",
        ),
        (
            "extern_first",
            "an extern declaration can precede an incomplete tentative definition",
            "extern struct S object;\
            struct S object;\
            struct S{\
                long n;\
            };\
            int main(void){\
                object.n=43;\
                return object.n!=43;\
            }",
        ),
        (
            "extern_after",
            "extern redeclarations retain tentative definition status",
            "union U object;\
            extern union U object;\
            union U{\
                long n;\
            };\
            extern union U object;\
            int main(void){\
                return object.n!=0;\
            }",
        ),
        (
            "qualified_repeated",
            "qualified tentative and extern declarations compose",
            "struct S;\
            const struct S object;\
            extern const struct S object;\
            const struct S object;\
            struct S{\
                long n;\
            };\
            int main(void){\
                return object.n!=0;\
            }",
        ),
        (
            "later_initializer",
            "a complete explicit initializer replaces tentative zero initialization",
            "struct S object;\
            struct S{\
                char c;\
                long n;\
            };\
            struct S object={\
                3,47};\
            int main(void){\
                return !(object.c==3&&object.n==47);\
            }",
        ),
        (
            "later_extern_initializer",
            "a later initialized extern declaration supplies the definition",
            "union U object;\
            union U{\
                long n;\
                double d;\
            };\
            extern union U object={53};\
            int main(void){\
                return object.n!=53;\
            }",
        ),
        (
            "later_const_initializer",
            "a qualified tentative object accepts a later complete initializer",
            "struct S;\
            const struct S object;\
            struct S{\
                long n;\
            };\
            const struct S object={59};\
            int main(void){\
                return object.n!=59;\
            }",
        ),
        (
            "tentative_after_initializer",
            "a later tentative redeclaration cannot erase an explicit initializer",
            "struct S object;\
            struct S{\
                int n;\
            };\
            struct S object={61};\
            struct S object;\
            extern struct S object;\
            int main(void){\
                return object.n!=61;\
            }",
        ),
] {
        cases.push(Case::new(
format!("tentative_{id}"),
purpose,
source,
Expectation::Exit(0),
));
    }
}
