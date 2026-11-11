/* Register and stack variable-argument traversal; scale point 12. Expected process status: zero. */
#include <stdarg.h>
long total(int count,...){va_list ap;long sum=0;int i;va_start(ap,count);for(i=0;i<count;i++)sum+=va_arg(ap,int);va_end(ap);return sum;}
int main(void)
{
    return total(13,18,25,32,39,46,53,60,67,74,81,88,95,102)!=780L;
}
