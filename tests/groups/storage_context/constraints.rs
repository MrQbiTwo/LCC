// Constraints cases for storage context.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "reject_register_scalar_address",
            "a register scalar cannot be the operand of address-of",
            "int main(void){register int n=3;int *p=&n;return *p;}",
        ),
        (
            "reject_register_pointer_address",
            "the register pointer object itself has no addressable C storage",
            "int main(void){int n=3;register int *p=&n;int **q=&p;return **q;}",
        ),
        (
            "reject_register_array_address",
            "explicit address-of cannot address a register array",
            "int main(void){register int a[3];int (*p)[3]=&a;return 0;}",
        ),
        (
            "reject_register_record_address",
            "explicit address-of cannot address a register record",
            "struct S{int n;};int main(void){register struct S s={3};struct S *p=&s;return 0;}",
        ),
        (
            "reject_register_member_address",
            "a direct dot member retains its register aggregate provenance",
            "struct S{int n;};int main(void){register struct S s={3};int *p=&s.n;return 0;}",
        ),
        (
            "reject_register_nested_member_address",
            "nested dot-member address checks reach the root register object",
            "struct I{\
                int n;\
            };\
            struct O{\
                struct I i;\
            };\
            int main(void){\
                register struct O s={{\
                    3}};\
                int *p=&s.i.n;\
                return 0;\
            }",
        ),
        (
            "reject_register_member_array_address",
            "a register record's array member cannot be explicitly addressed",
            "struct S{int a[3];};int main(void){register struct S s;int (*p)[3]=&s.a;return 0;}",
        ),
        (
            "reject_register_sizeof_address",
            "unevaluated sizeof still applies the address-of constraint",
            "int main(void){register int n=3;return sizeof(&n);}",
        ),
        (
            "reject_register_sizeof_member_address",
            "sizeof does not make a register record member addressable",
            "struct S{int n;};int main(void){register struct S s;return sizeof(&s.n);}",
        ),
        (
            "reject_register_parameter_address",
            "register scalar parameters retain address restrictions in the body",
            "int f(register int n){int *p=&n;return *p;}int main(void){return f(3);}",
        ),
        (
            "reject_register_pointer_parameter_address",
            "a register pointer parameter cannot itself be addressed",
            "int f(register int *p){int **q=&p;return 0;}int main(void){return 0;}",
        ),
        (
            "reject_register_array_parameter_address",
            "array-parameter adjustment preserves register storage",
            "int f(register int a[3]){int **q=&a;return 0;}int main(void){return 0;}",
        ),
        (
            "reject_register_function_parameter_address",
            "function-parameter adjustment preserves register storage",
            "int f(register int fn(int)){int (**p)(int)=&fn;return 0;}int main(void){return 0;}",
        ),
        (
            "reject_outer_register_nested_parameter_address",
            "nested ordinary parameter declspec must not clear outer register storage",
            "int f(register int (*fn)(int n)){int (**p)(int)=&fn;return 0;}int main(void){return 0;}",
        ),
        (
            "reject_outer_register_nested_register_address",
            "both levels of nested register parameter declarations retain the outer constraint",
            "int f(register int (*fn)(register int n)){\
                int (**p)(int)=&fn;\
                return 0;\
            }\
            int main(void){\
                return 0;\
            }",
        ),
        (
            "reject_register_comma_declarator",
            "register storage applies to every declarator in the declaration",
            "int main(void){register int a=3,b=7;int *p=&b;return 0;}",
        ),
        (
            "reject_register_restored_scope",
            "leaving an automatic shadow restores the outer register restriction",
            "int main(void){register int n=3;{int n=7;int *p=&n;}return sizeof(&n);}",
        ),
        (
            "reject_ordinary_prototype_register_definition",
            "definition register storage survives composition with an ordinary prototype",
            "int f(int);int f(register int n){int *p=&n;return *p;}int main(void){return 0;}",
        ),
        (
            "reject_register_self_initializer_address",
            "register storage is installed before parsing the object's initializer",
            "int main(void){register int *p=(int*)&p;return 0;}",
        ),
        (
            "reject_register_qualified_address",
            "qualifiers do not erase a register object's address restriction",
            "int main(void){register const volatile int n=3;return sizeof(&n);}",
        ),
        (
            "reject_register_union_member_address",
            "union dot members retain the root register object's address restriction",
            "union U{int n;long m;};int main(void){register union U u={3};int *p=&u.n;return 0;}",
        ),
    ] { cases.push(Case::new(
        format!("storage_context_{}", id),
        purpose,
        source,
        Expectation::Reject("E008".into()),
    )); }

    for storage in ["static", "extern", "typedef", "auto", "register"] {
        for (context, source) in [
            (
                "cast",
                format!("int main(void){{return ({storage} int)3;}}"),
            ),
            (
                "sizeof",
                format!("int main(void){{return sizeof({storage} int);}}"),
            ),
            (
                "member",
                format!("struct S{{{storage} int n;}};int main(void){{return 0;}}"),
            ),
        ] {
            cases.push(Case::new(
                format!("storage_context_{}", &format!("reject_{storage}_{context}")),
                "storage-class specifiers are not permitted in a type name or record member specifier list",
                &source,
                Expectation::Reject("E008".into()),
            ));
        }
    }
    for (id, purpose, source) in [
        (
            "reject_cast_named_scalar",
            "a type name cannot introduce a scalar object identifier",
            "int main(void){return (int n)3;}",
        ),
        (
            "reject_cast_named_pointer",
            "a type name cannot introduce a pointer object identifier",
            "int main(void){int n=3;return *(int *p)&n;}",
        ),
        (
            "reject_cast_named_function_pointer",
            "an outer identifier is forbidden even when nested parameter names are legal",
            "int add(int n){return n+3;}int main(void){return ((int (*p)(int n))add)(4);}",
        ),
        (
            "reject_sizeof_named_scalar",
            "sizeof type syntax does not declare a scalar object",
            "int main(void){return sizeof(int n);}",
        ),
        (
            "reject_sizeof_named_array",
            "sizeof type syntax does not declare an array object",
            "int main(void){return sizeof(int a[3]);}",
        ),
        (
            "reject_sizeof_named_function_pointer",
            "sizeof rejects an outer function-pointer identifier",
            "int main(void){return sizeof(int (*p)(int n));}",
        ),
    ] {
        cases.push(Case::new(
            format!("storage_context_{}", id),
            purpose,
            source,
            Expectation::Reject("E008".into()),
        ));
    }
}
