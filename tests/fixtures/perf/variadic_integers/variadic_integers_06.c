/* Register and stack variable-argument traversal; scale point 6. Expected process status: zero. */
#include <stdarg.h>
long total(int count,...){va_list ap;long sum=0;int i;va_start(ap,count);for(i=0;i<count;i++)sum+=va_arg(ap,int);va_end(ap);return sum;}
int main(void)
{
    return total(7,12,19,26,33,40,47,54)!=231L;
}
