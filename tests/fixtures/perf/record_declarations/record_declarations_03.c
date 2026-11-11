/* Tag/layout lookup growth and offsetof constants; scale point 3. Expected process status: zero. */
#include <stddef.h>
struct Record0{char tag;long value;int samples[1];};
struct Record1{char tag;long value;int samples[2];};
struct Record2{char tag;long value;int samples[3];};
struct Record3{char tag;long value;int samples[4];};
struct Record4{char tag;long value;int samples[5];};
struct Record5{char tag;long value;int samples[6];};
struct Record6{char tag;long value;int samples[7];};
struct Record7{char tag;long value;int samples[8];};
int main(void){size_t sum=0;
sum+=offsetof(struct Record0,value);
sum+=offsetof(struct Record1,value);
sum+=offsetof(struct Record2,value);
sum+=offsetof(struct Record3,value);
sum+=offsetof(struct Record4,value);
sum+=offsetof(struct Record5,value);
sum+=offsetof(struct Record6,value);
sum+=offsetof(struct Record7,value);
return sum!=64UL;}
