// Constraints cases for aggregate.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (purpose, source, code) in [
        (
            "incomplete pointer increment",
            "struct S;int main(void){struct S *p;p++;return 0;}",
            "E008",
        ),
        (
            "incomplete pointer prefix increment",
            "struct S;int main(void){struct S *p;++p;return 0;}",
            "E008",
        ),
        (
            "incomplete array indexing",
            "struct S;int main(void){struct S *p;p[1];return 0;}",
            "E008",
        ),
        (
            "incomplete object",
            "struct S;struct S s;int main(void){return 0;}",
            "E008",
        ),
        (
            "incomplete sizeof",
            "struct S;int main(void){return sizeof(struct S);}",
            "E008",
        ),
        (
            "incomplete field",
            "struct S{struct S s;};int main(void){return 0;}",
            "E008",
        ),
        (
            "incomplete array element",
            "struct S;struct S a[2];int main(void){return 0;}",
            "E008",
        ),
        (
            "duplicate member",
            "struct S{int x;long x;};int main(void){return 0;}",
            "E009",
        ),
        (
            "duplicate definition",
            "struct S{int x;};struct S{int y;};int main(void){return 0;}",
            "E009",
        ),
        (
            "conflicting tag kind",
            "struct S;union S *p;int main(void){return 0;}",
            "E008",
        ),
        (
            "missing member",
            "struct S{int x;};int main(void){struct S s;return s.y;}",
            "E008",
        ),
        (
            "arrow requires pointer",
            "struct S{int x;};int main(void){struct S s;return s->x;}",
            "E008",
        ),
        (
            "dot requires aggregate",
            "int main(void){int x;return x.y;}",
            "E008",
        ),
        (
            "different tag assignment",
            "struct A{int x;};struct B{int x;};int main(void){struct A a;struct B b;a=b;return 0;}",
            "E008",
        ),
        (
            "aggregate scalar assignment",
            "struct S{int x;};int main(void){struct S s;s=7;return 0;}",
            "E008",
        ),
        (
            "aggregate compound assignment",
            "struct S{int x;};int main(void){struct S s,t;s+=t;return 0;}",
            "E008",
        ),
        (
            "aggregate arithmetic",
            "struct S{int x;};int main(void){struct S s,t;s+t;return 0;}",
            "E008",
        ),
        (
            "aggregate condition",
            "struct S{int x;};int main(void){struct S s;if(s)return 1;return 0;}",
            "E008",
        ),
        (
            "aggregate prefix increment",
            "struct S{int x;};int main(void){struct S s;++s;return 0;}",
            "E008",
        ),
        (
            "aggregate postfix increment",
            "struct S{int x;};int main(void){struct S s;s++;return 0;}",
            "E008",
        ),
        (
            "const aggregate member",
            "struct S{int x;};int main(void){const struct S s;s.x=1;return 0;}",
            "E008",
        ),
        (
            "const pointer target member",
            "struct S{int x;};int main(void){const struct S *p;p->x=1;return 0;}",
            "E008",
        ),
        (
            "const member whole copy",
            "struct S{const int x;};int main(void){struct S s,t;s=t;return 0;}",
            "E008",
        ),
        (
            "nested const member whole copy",
            "struct S{const int x;};struct T{struct S a[2];};int main(void){struct T s,t;s=t;return 0;}",
            "E008",
        ),
        (
            "const aggregate array element",
            "struct S{int a[2];};int main(void){const struct S s;s.a[0]=1;return 0;}",
            "E008",
        ),
        (
            "void field",
            "struct S{void x;};int main(void){return 0;}",
            "E008",
        ),
        (
            "function field",
            "struct S{int f(void);};int main(void){return 0;}",
            "E008",
        ),
        (
            "field storage class",
            "struct S{static int x;};int main(void){return 0;}",
            "E008",
        ),
        (
            "incomplete pointer arithmetic",
            "struct S;int main(void){struct S *p;p+1;return 0;}",
            "E008",
        ),
        (
            "incomplete aggregate result",
            "struct S;struct S f(void){for(;;){}}int main(void){return 0;}",
            "E008",
        ),
    ] {
        cases.push(Case::new(
            format!("aggregate_{:03}",cases.len()+1),
            purpose,
            source,
            Expectation::Reject(code.into()),
        ));
    }
}
