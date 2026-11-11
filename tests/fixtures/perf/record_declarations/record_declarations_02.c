/* Tag/layout lookup growth and offsetof constants; scale point 2. Expected process status: zero. */
#include <stddef.h>
struct Record0{char tag;long value;int samples[1];};
struct Record1{char tag;long value;int samples[2];};
struct Record2{char tag;long value;int samples[3];};
struct Record3{char tag;long value;int samples[4];};
int main(void){size_t sum=0;
sum+=offsetof(struct Record0,value);
sum+=offsetof(struct Record1,value);
sum+=offsetof(struct Record2,value);
sum+=offsetof(struct Record3,value);
return sum!=32UL;}
