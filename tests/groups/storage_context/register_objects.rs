// Register objects cases for storage context.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (id, purpose, source) in [
        (
            "register_scalar",
            "register objects retain normal reads, stores, and compound operations",
            "int main(void){register int n=3;n+=4;++n;return n!=8;}",
        ),
        (
            "register_narrow",
            "register narrow integers retain conversions and promotion",
            "int main(void){\
                register unsigned char n=255;\
                register short s=-7;\
                n++;\
                s+=9;\
                return !(n==0&&s==2);\
            }",
        ),
        (
            "register_floating",
            "register float and double use normal arithmetic and conversion",
            "int main(void){\
                register float a=1.5f;\
                register double b=2.25;\
                a+=1.0f;\
                b*=2.0;\
                return !(a==2.5f&&b==4.5);\
            }",
        ),
        (
            "register_sizeof_scalar",
            "sizeof does not require the address of a register scalar",
            "int main(void){register int n=3;return !(sizeof(n)==4&&sizeof(n++)==4&&n==3);}",
        ),
        (
            "register_sizeof_array",
            "sizeof a register array avoids array-to-pointer conversion",
            "int main(void){register int a[3]={1,2,3};return sizeof(a)!=12;}",
        ),
        (
            "register_sizeof_record",
            "register aggregates remain valid sizeof operands",
            "struct S{\
                int n;\
                long m;\
            };\
            int main(void){\
                register struct S s={\
                    3,7};\
                return !(sizeof(s)==16&&sizeof(s.m)==8);\
            }",
        ),
        (
            "register_record_value",
            "register records support member reads, stores, and value copies",
            "struct S{\
                int a;\
                int b;\
            };\
            int main(void){\
                register struct S s={\
                    3,7};\
                struct S t;\
                s.a++;\
                t=s;\
                return !(t.a==4&&t.b==7);\
            }",
        ),
        (
            "register_nested_record",
            "nested dot members of register records can be modified without taking an address",
            "struct I{\
                int n;\
            };\
            struct O{\
                struct I i;\
                int m;\
            };\
            int main(void){\
                register struct O s={{\
                    3},7};\
            s.i.n+=2;\
            return !(s.i.n==5&&s.m==7);\
            }",
        ),
        (
            "register_pointer_pointee",
            "addressing a dereferenced register pointer addresses its pointee",
            "int main(void){int n=3;register int *p=&n;int *q=&*p;*q=7;return !(q==p&&n==7);}",
        ),
        (
            "register_pointer_arrow",
            "an arrow member belongs to the pointed-to record rather than the register pointer",
            "struct S{\
                int n;\
            };\
            int main(void){\
                struct S s={3};\
                register struct S *p=&s;\
                int *q=&p->n;\
                *q=7;\
                return s.n!=7;\
            }",
        ),
        (
            "register_pointer_nested_arrow",
            "nested dot members after an arrow remain addressable",
            "struct I{\
                int n;\
            };\
            struct O{\
                struct I i;\
            };\
            int main(void){\
                struct O s={{\
                    3}};\
                register struct O *p=&s;\
                int *q=&p->i.n;\
                *q=9;\
                return s.i.n!=9;\
            }",
        ),
        (
            "register_record_pointer_member",
            "a pointer loaded from a register record may address its pointee",
            "struct S{\
                int *p;\
            };\
            int main(void){\
                int n=3;\
                register struct S s;\
                int *q;\
                s.p=&n;\
                q=&*s.p;\
                *q=9;\
                return n!=9;\
            }",
        ),
        (
            "register_subscript_pointee",
            "array indexing through a register pointer leaves elements addressable",
            "int main(void){int a[3]={1,3,7};register int *p=a;int *q=&p[2];*q=9;return a[2]!=9;}",
        ),
        (
            "register_parameter_value",
            "register parameter storage preserves ordinary call and mutation behavior",
            "int f(register int n){n+=3;return n;}int main(void){return f(4)!=7;}",
        ),
        (
            "register_parameter_pointee",
            "register pointer parameters permit pointee address formation",
            "int f(register int *p){\
                int *q=&*p;\
                *q+=3;\
                return *p;\
            }\
            int main(void){\
                int n=4;\
                return !(f(&n)==7&&n==7);\
            }",
        ),
        (
            "register_array_parameter",
            "array parameters adjust to register pointers and retain accessible elements",
            "int f(register int a[3]){\
                int *p=&a[1];\
                return *p+sizeof(a);\
            }\
            int main(void){\
                int a[3]={\
                    1,3,7};\
                return f(a)!=11;\
            }",
        ),
        (
            "register_function_parameter",
            "function parameters adjust to callable register pointers",
            "int add(int n){\
                return n+3;\
            }\
            int f(register int fn(int n)){\
                return fn(4);\
            }\
            int main(void){\
                return f(add)!=7;\
            }",
        ),
        (
            "register_shadow_inner_auto",
            "an automatic inner shadow remains addressable inside a register scope",
            "int main(void){register int n=3;int r;{int n=7;int *p=&n;*p=9;r=n;}return !(r==9&&n==3);}",
        ),
        (
            "register_shadow_restores_auto",
            "leaving a register shadow restores addressability of an outer automatic",
            "int main(void){int n=3;int *p;{register int n=7;n++;}p=&n;*p=9;return n!=9;}",
        ),
        (
            "register_parameter_shadow",
            "an ordinary nested object can shadow a register parameter",
            "int f(register int n){\
                int r;\
                {\
                    int n=7;\
                    int *p=&n;\
                    r=*p;\
                }\
                return n+r;\
            }\
            int main(void){\
                return f(3)!=10;\
            }",
        ),
        (
            "register_storage_no_leak",
            "register state does not leak to the following declaration",
            "int main(void){register int n=3;int m=7;int *p=&m;return !(n==3&&*p==7);}",
        ),
        (
            "nested_parameter_no_leak",
            "nested register parameter storage does not mark its enclosing pointer parameter",
            "int add(int n){\
                return n+3;\
            }\
            int f(int (*fn)(register int n)){\
                int (**p)(int)=&fn;\
                return (*p)(4);\
            }\
            int main(void){\
                return f(add)!=7;\
            }",
        ),
        (
            "nested_local_no_leak",
            "nested register parameters do not mark an automatic function pointer object",
            "int add(int n){\
                return n+3;\
            }\
            int main(void){\
                int (*fn)(register int n)=add;\
                int (**p)(int)=&fn;\
                return (*p)(4)!=7;\
            }",
        ),
        (
            "outer_register_nested_parameter",
            "outer register storage survives a nested ordinary parameter declaration",
            "int add(int n){\
                return n+3;\
            }\
            int f(register int (*fn)(int n)){\
                return fn(4);\
            }\
            int main(void){\
                return f(add)!=7;\
            }",
        ),
        (
            "nested_type_name_parameters",
            "type names may contain named and register function parameters",
            "int add(int n){\
                return n+3;\
            }\
            int main(void){\
                int (*p)(int)=((int (*)(register int n))add);\
                return !(p(4)==7&&sizeof(int (*)(int n))==8);\
            }",
        ),
        (
            "ordinary_address_controls",
            "automatic, static, and extern objects retain addressability",
            "int x=3;\
            int main(void){\
                auto int a=5;\
                static int b=7;\
                extern int x;\
                int *p=&a;\
                int *q=&b;\
                int *r=&x;\
                return !(*p==5&&*q==7&&*r==3);\
            }",
        ),
        (
            "register_prototype_ordinary_definition",
            "prototype register storage does not restrict the later definition's ordinary parameter",
            "int f(register int);int f(int n){int *p=&n;return *p+3;}int main(void){return f(4)!=7;}",
        ),
    ] { cases.push(Case::new(format!("storage_context_{}", id), purpose, source, Expectation::Exit(0))); }
}
