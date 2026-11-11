// Foreign endpoints verify block linkage and persistent private storage.
use crate::{Case, Expectation};

// Swap provider/consumer across the compiler boundary for each storage contract.
pub fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for (name, provider, consumer) in [
        (
            "block_object",
            "int value=7;",
            "int driver(void){extern int value;value+=2;return value==9;}",
        ),
        (
            "block_array",
            "int values[]={3,5,7};",
            "int driver(void){extern int values[3];return sizeof(values)==12&&values[2]==7;}",
        ),
        (
            "block_function",
            "int add(int n){return n+7;}",
            "int driver(void){int add(int);return add(3)==10;}",
        ),
        (
            "extern_function",
            "int add(int n){return n+9;}",
            "int driver(void){extern int add(int);return add(3)==12;}",
        ),
        (
            "private_counter",
            "int next(void){static int count=3;return ++count;}",
            "int count=99;\
            int next(void);\
            int driver(void){\
                int a=next();\
                int b=next();\
                return a==4&&b==5&&count==99;\
            }",
        ),
        (
            "private_address",
            "int *get(void){static int value=7;return &value;}",
            "int *get(void);int driver(void){int *p=get();*p=13;return get()==p&&*get()==13;}",
        ),
        (
            "private_string",
            "char *get(void){static char text[]=\"ab\";return text;}",
            "char *get(void);\
            int driver(void){\
                char *p=get();\
                p[1]='z';\
                return get()==p&&get()[0]=='a'&&get()[1]=='z'&&get()[2]==0;\
            }",
        ),
        (
            "private_external_address",
            "int *get(void){extern int value;static int *p=&value;return p;}",
            "int value=17;int *get(void);int driver(void){return get()==&value&&*get()==17;}",
        ),
        (
            "private_binary80",
            "long double *get(void){static long double a[]={1.25L,2.5L};return a;}",
            "long double *get(void);\
            int driver(void){\
                long double *p=get();\
                p[1]+=1.0L;\
                return get()==p&&p[0]==1.25L&&get()[1]==3.5L;\
            }",
        ),
    ] {
        for (direction, source, foreign) in
            [("in", provider, consumer), ("out", consumer, provider)]
        {
            cases.push(Case::new(
                format!("interop_storage_{name}_{direction}"),
                format!("independent block storage/linkage {name} {direction}"),
                source,
                Expectation::CInterop(foreign.into()),
            ));
        }
    }
    cases
}
