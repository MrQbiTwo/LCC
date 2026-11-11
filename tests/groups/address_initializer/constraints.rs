// Constraints cases for address initializer.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, code) in [
        (
            "pointer variable load",
            "int x;int *p=&x;int *q=p;int main(void){return 0;}",
            "E008",
        ),
        (
            "scalar variable displacement",
            "int a[3];int n=1;int *p=a+n;int main(void){return 0;}",
            "E008",
        ),
        (
            "pointer load under cast",
            "int x;int *p=&x;long n=(long)p;int main(void){return 0;}",
            "E008",
        ),
        (
            "pointer array element load",
            "int x;int *a[]={&x};int *p=a[0];int main(void){return 0;}",
            "E008",
        ),
        (
            "pointer member load",
            "int x;struct S{int *p;};struct S s={&x};int *p=s.p;int main(void){return 0;}",
            "E008",
        ),
        (
            "const pointer object load",
            "int x;int *const p=&x;int *q=p;int main(void){return 0;}",
            "E008",
        ),
        (
            "arrow through pointer object load",
            "struct S{int x;};struct S s;struct S *p=&s;int *q=&p->x;int main(void){return 0;}",
            "E008",
        ),
        (
            "function call initializer",
            "int x;int *f(void){return &x;}int *p=f();int main(void){return 0;}",
            "E008",
        ),
        (
            "function call displacement",
            "int a[3];int f(void){return 1;}int *p=a+f();int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic condition",
            "int x,y;int *p=&x?&x:&y;int main(void){return 0;}",
            "E008",
        ),
        (
            "variable condition",
            "int x,y,n=1;int *p=n?&x:&y;int main(void){return 0;}",
            "E008",
        ),
        (
            "narrow symbolic integer",
            "int x;unsigned int n=(unsigned int)&x;int main(void){return 0;}",
            "E008",
        ),
        (
            "narrow then widen symbolic integer",
            "int x;unsigned long n=(unsigned long)(unsigned int)&x;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic destination truncation",
            "int x;unsigned int n=(unsigned long)&x;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic floating cast",
            "int x;double n=(double)(long)&x;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic multiplication",
            "int x;unsigned long n=(unsigned long)&x*2;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic bitwise operation",
            "int x;unsigned long n=(unsigned long)&x|1;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic right shift",
            "int x;unsigned long n=(unsigned long)&x>>1;int main(void){return 0;}",
            "E008",
        ),
        (
            "two symbolic operands",
            "int x,y;unsigned long n=(unsigned long)&x+(unsigned long)&y;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic integer negation",
            "int x;long n=-(long)&x;int main(void){return 0;}",
            "E008",
        ),
        (
            "address of bit-field",
            "struct S{unsigned x:3;};struct S s;unsigned *p=&s.x;int main(void){return 0;}",
            "E008",
        ),
        (
            "symbolic pointer increment",
            "int x;int *p=(&x)++;int main(void){return 0;}",
            "E008",
        ),
        (
            "unresolved external data in executable",
            "extern int x;int *p=&x;int main(void){return 0;}",
            "E009",
        ),
        (
            "unresolved external function in executable",
            "int f(int);int (*p)(int)=f;int main(void){return 0;}",
            "E009",
        ),
        (
            "scaled addend overflow",
            "int a;int *p=&a+0x4000000000000000L;int main(void){return 0;}",
            "E011",
        ),
        (
            "signed addend overflow",
            "char a;char *p=(&a+0x7fffffffffffffffL)+1;int main(void){return 0;}",
            "E011",
        ),
        (
            "unsigned offset outside addend range",
            "char a;char *p=&a+0xffffffffffffffffUL;int main(void){return 0;}",
            "E011",
        ),
        (
            "resolved address underflow",
            "char a;char *p=&a+(-9223372036854775807L-1);int main(void){return 0;}",
            "E011",
        ),
        (
            "numeric address overflow",
            "char *p=(char*)0xffffffffffffffffUL+1;int main(void){return 0;}",
            "E011",
        ),
        (
            "numeric address underflow",
            "char *p=(char*)0-1;int main(void){return 0;}",
            "E011",
        ),
        (
            "incompatible conditional pointer arms",
            "int a;double b;void *p=1?&a:&b;int main(void){return 0;}",
            "E008",
        ),
        (
            "conditional pointer nonzero integer",
            "int a;int *p=1?&a:2;int main(void){return 0;}",
            "E008",
        ),
        (
            "conditional pointer variable integer",
            "int main(void){int a,n=0;int *p=1?&a:n;return 0;}",
            "E008",
        ),
        (
            "conditional pointee const is preserved",
            "int main(void){int a=1;const int b=2;*(1?&a:&b)=3;return 0;}",
            "E008",
        ),
        (
            "qualified void zero keeps void result",
            "int x;int main(void){*(1?&x:(const void*)0)=3;return x;}",
            "E008",
        ),
        (
            "qualified void zero keeps void result in left arm",
            "int x;int main(void){*(0?(const void*)0:&x)=3;return x;}",
            "E008",
        ),
        (
            "const void zero is not a function null arm",
            "int f(void){return 3;}int (*p)(void)=1?f:(const void*)0;int main(void){return p();}",
            "E008",
        ),
        (
            "volatile void zero is not a function null arm",
            "int f(void){return 3;}int (*p)(void)=0?(volatile void*)0:f;int main(void){return p();}",
            "E008",
        ),
    ] {
        cases.push(Case::new(
            format!("address_initializer_{:03}", cases.len()+1),
            purpose,
            source,
            Expectation::Reject(code.into()),
        ));
    }
}
