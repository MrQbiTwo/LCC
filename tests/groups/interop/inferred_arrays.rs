// Inferred arrays cases for interop.
use crate::{Case, Expectation};

// Append declarative cases in their established catalog order.
pub(super) fn append(cases: &mut Vec<Case>) {
    // Foreign consumers check inferred data layout independently of LCC lowering.
    for (name, producer, consumer) in [
        (
            "inferred_rows",
            "int a[][3]={1,2,3,4};int size(void){return sizeof(a);}",
            "extern int a[][3];\
            int size(void);\
            int driver(void){\
                return size()==24&&a[0][2]==3&&a[1][0]==4&&a[1][1]==0&&a[1][2]==0;\
            }",
        ),
        (
            "inferred_string",
            "char a[]=\"a\\0b\" \"c\";int size(void){return sizeof(a);}",
            "extern char a[];\
            int size(void);\
            int driver(void){\
                return size()==5&&a[0]=='a'&&a[1]==0&&a[2]=='b'&&a[3]=='c'&&a[4]==0;\
            }",
        ),
        (
            "inferred_bitfields",
            "struct S{\
                unsigned x:3;\
                signed int y:5;\
                char z;\
            };\
            struct S a[]={{\
                7,-2,9},{\
                3,4,5}};\
            int size(void){\
                return sizeof(a);\
            }",
            "struct S{\
                unsigned x:3;\
                signed int y:5;\
                char z;\
            };\
            extern struct S a[];\
            int size(void);\
            int driver(void){\
                return size()==2*sizeof(struct S)&&a[0].x==7&&a[0].y==-2&&a[0].z==9&&a[1].x==3&&a[1].y==4&&\
                a[1].z==5;\
            }",
        ),
        (
            "inferred_binary80",
            "long double a[]={1.25L,2.5L,3.75L};int size(void){return sizeof(a);}",
            "extern long double a[];\
            int size(void);\
            int driver(void){\
                return size()==48&&a[0]==1.25L&&a[1]==2.5L&&a[2]==3.75L;\
            }",
        ),
        (
            "inferred_typedef",
            "typedef int A[];A a={1};A b={2,3};int size(void){return sizeof(a)+sizeof(b);}",
            "extern int a[],b[];\
            int size(void);\
            int driver(void){\
                return size()==12&&a[0]==1&&b[0]==2&&b[1]==3;\
            }",
        ),
        (
            "inferred_tentative",
            "int a[];int read(void){return a[0];}",
            "extern int a[];int read(void);int driver(void){int x=a[0];a[0]=7;return x==0&&read()==7;}",
        ),
    ] {
        for (direction, source, foreign) in [("in", producer, consumer), ("out", consumer, producer)] {
            cases.push(Case::new(
                format!("interop_{name}_{direction}"),
                format!("foreign inferred array data layout {name} {direction}"),
                source,
                Expectation::CInterop(foreign.into()),
            ));
        }
    }
}
