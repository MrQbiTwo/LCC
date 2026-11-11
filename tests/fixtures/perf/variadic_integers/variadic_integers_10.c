/* Register and stack variable-argument traversal; scale point 10. Expected process status: zero. */
#include <stdarg.h>
long total(int count,...){va_list ap;long sum=0;int i;va_start(ap,count);for(i=0;i<count;i++)sum+=va_arg(ap,int);va_end(ap);return sum;}
int main(void)
{
    return total(11,16,23,30,37,44,51,58,65,72,79,86)!=561L;
}
