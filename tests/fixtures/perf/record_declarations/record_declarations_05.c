/* Tag/layout lookup growth and offsetof constants; scale point 5. Expected process status: zero. */
#include <stddef.h>
struct Record0{char tag;long value;int samples[1];};
struct Record1{char tag;long value;int samples[2];};
struct Record2{char tag;long value;int samples[3];};
struct Record3{char tag;long value;int samples[4];};
struct Record4{char tag;long value;int samples[5];};
struct Record5{char tag;long value;int samples[6];};
struct Record6{char tag;long value;int samples[7];};
struct Record7{char tag;long value;int samples[8];};
struct Record8{char tag;long value;int samples[9];};
struct Record9{char tag;long value;int samples[1];};
struct Record10{char tag;long value;int samples[2];};
struct Record11{char tag;long value;int samples[3];};
struct Record12{char tag;long value;int samples[4];};
struct Record13{char tag;long value;int samples[5];};
struct Record14{char tag;long value;int samples[6];};
struct Record15{char tag;long value;int samples[7];};
int main(void){size_t sum=0;
sum+=offsetof(struct Record0,value);
sum+=offsetof(struct Record1,value);
sum+=offsetof(struct Record2,value);
sum+=offsetof(struct Record3,value);
sum+=offsetof(struct Record4,value);
sum+=offsetof(struct Record5,value);
sum+=offsetof(struct Record6,value);
sum+=offsetof(struct Record7,value);
sum+=offsetof(struct Record8,value);
sum+=offsetof(struct Record9,value);
sum+=offsetof(struct Record10,value);
sum+=offsetof(struct Record11,value);
sum+=offsetof(struct Record12,value);
sum+=offsetof(struct Record13,value);
sum+=offsetof(struct Record14,value);
sum+=offsetof(struct Record15,value);
return sum!=128UL;}
