// Values and calls cases for interop.
use super::pair;
use crate::Case;

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    for (name, decl, ty, init, check) in [
        (
            "ii",
            "struct S{long x,y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "ss",
            "struct S{double x,y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "is",
            "struct S{long x;double y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "si",
            "struct S{double x;long y;};",
            "struct S",
            "{3,4}",
            "s.x==3&&s.y==4",
        ),
        (
            "float_array",
            "struct S{float x[3];};",
            "struct S",
            "{{1,2,4}}",
            "s.x[0]==1&&s.x[1]==2&&s.x[2]==4",
        ),
        (
            "memory",
            "struct S{long x,y,z;};",
            "struct S",
            "{1,2,4}",
            "s.x==1&&s.y==2&&s.z==4",
        ),
        (
            "binary80_record",
            "struct S{long double x;};",
            "struct S",
            "{3.25L}",
            "s.x==3.25L",
        ),
        (
            "union",
            "union S{long x;double y;};",
            "union S",
            "{7}",
            "s.x==7",
        ),
        (
            "bitfield",
            "struct S{char c;unsigned a:4,b:12;char tail;};",
            "struct S",
            "{18,5,0xabc,52}",
            "s.c==18&&s.a==5&&s.b==0xabc&&s.tail==52",
        ),
        (
            "mixed_bitfield",
            "struct S{double x;unsigned a:3;signed int b:5;};",
            "struct S",
            "{3.25,5,-7}",
            "s.x==3.25&&s.a==5&&s.b==-7",
        ),
        (
            "unnamed",
            "struct S{unsigned :3;char c;};",
            "struct S",
            "{7}",
            "s.c==7",
        ),
        (
            "zero_union",
            "union S{unsigned :0;char c;};",
            "union S",
            "{7}",
            "s.c==7",
        ),
    ] {
        pair(
            cases,
            &format!("value_{name}"),
            decl,
            &format!("{ty} target({ty} s)"),
            "return s;",
            &format!("{ty} s={init};s=target(s);return {check};"),
        );
        pair(
            cases,
            &format!("vararg_{name}"),
            decl,
            "int target(int n,...)",
            &format!(
                "va_list a;\
                {ty} s;\
                int tail;\
                va_start(a,n);\
                s=va_arg(a,{ty});\
                tail=va_arg(a,int);\
                va_end(a);\
                return ({check})&&tail==9;"
            ),
            &format!("{ty} s={init};return target(1,s,9);"),
        );
    }
    pair(
        cases,
        "scalar_mixed",
        "",
        "int target(int n,...)",
        "va_list a;\
        int x;\
        unsigned int y;\
        long z;\
        double d;\
        long double l;\
        int *p;\
        va_start(a,n);\
        x=va_arg(a,int);\
        y=va_arg(a,unsigned int);\
        z=va_arg(a,long);\
        d=va_arg(a,double);\
        l=va_arg(a,long double);\
        p=va_arg(a,int *);\
        va_end(a);\
        return x==-3&&y==4294967295U&&z==-9&&d==3.25&&l==4.5L&&*p==7;",
        "signed char x=-3;float y=3.25f;int z=7;return target(6,x,4294967295U,-9L,y,4.5L,&z);",
    );
    pair(
        cases,
        "overflow",
        "",
        "int target(int n,...)",
        "va_list a;\
        int i,x;\
        double d;\
        long double l;\
        long total=0;\
        va_start(a,n);\
        for(i=0;i<10;i++){\
            x=va_arg(a,int);\
            d=va_arg(a,double);\
            total+=x+(int)d;\
        }\
        l=va_arg(a,long double);\
        x=va_arg(a,int);\
        va_end(a);\
        return total==110&&l==3.25L&&x==9;",
        "return target(10,1,1.,2,2.,3,3.,4,4.,5,5.,6,6.,7,7.,8,8.,9,9.,10,10.,3.25L,9);",
    );
    pair(
        cases,
        "gp_rollback",
        "struct S{long x,y;};",
        "int target(long a,long b,long c,long d,long n,...)",
        "va_list v;\
        struct S s;\
        long last;\
        va_start(v,n);\
        s=va_arg(v,struct S);\
        last=va_arg(v,long);\
        va_end(v);\
        return a+b+c+d+n==15&&s.x==6&&s.y==7&&last==9;",
        "struct S s={6,7};return target(1,2,3,4,5,s,9L);",
    );
    pair(
        cases,
        "sse_rollback",
        "struct S{double x,y;};",
        "int target(double a,double b,double c,double d,double e,double f,double n,...)",
        "va_list v;\
        struct S s;\
        double last;\
        va_start(v,n);\
        s=va_arg(v,struct S);\
        last=va_arg(v,double);\
        va_end(v);\
        return a+b+c+d+e+f+n==28&&s.x==6&&s.y==7&&last==9;",
        "struct S s={6,7};return target(1.,2.,3.,4.,5.,6.,7.,s,9.);",
    );
    pair(
        cases,
        "mixed_rollback",
        "struct S{double x;long y;};",
        "int target(long a,long b,long c,long d,long e,long n,...)",
        "va_list v;\
        struct S s;\
        double last;\
        va_start(v,n);\
        s=va_arg(v,struct S);\
        last=va_arg(v,double);\
        va_end(v);\
        return a+b+c+d+e+n==21&&s.x==6&&s.y==7&&last==9;",
        "struct S s={6,7};return target(1,2,3,4,5,6,s,9.);",
    );
    pair(
        cases,
        "hidden_result",
        "struct S{long x,y,z;};",
        "struct S target(int n,...)",
        "va_list a;\
        struct S s;\
        int i;\
        s.x=0;\
        va_start(a,n);\
        for(i=0;i<7;i++)s.x+=va_arg(a,int);\
        s.y=va_arg(a,long);\
        s.z=(long)va_arg(a,long double);\
        va_end(a);\
        return s;",
        "struct S s=target(7,1,2,3,4,5,6,7,9L,3.0L);return s.x==28&&s.y==9&&s.z==3;",
    );
    pair(
        cases,
        "named_stack",
        "struct S{long x,y,z;};",
        "int target(int a,int b,int c,int d,int e,int f,int g,struct S n,...)",
        "va_list v;\
        long double x;\
        int y;\
        va_start(v,n);\
        x=va_arg(v,long double);\
        y=va_arg(v,int);\
        va_end(v);\
        return a+b+c+d+e+f+g==28&&n.x==1&&n.y==2&&n.z==3&&x==4.5L&&y==9;",
        "struct S s={1,2,3};return target(1,2,3,4,5,6,7,s,4.5L,9);",
    );
    pair(
        cases,
        "copy_restart",
        "",
        "int target(int n,...)",
        "va_list a,b;\
        int i,x,y;\
        long total=0;\
        va_start(a,n);\
        for(i=0;i<7;i++)total+=va_arg(a,int);\
        va_copy(b,a);\
        x=va_arg(a,int);\
        y=va_arg(b,int);\
        va_end(a);\
        va_end(b);\
        va_start(a,n);\
        i=va_arg(a,int);\
        va_end(a);\
        return total==28&&x==8&&y==8&&i==1;",
        "return target(8,1,2,3,4,5,6,7,8);",
    );
}
