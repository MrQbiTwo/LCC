/* Register and stack variable-argument traversal; scale point 8. Expected process status: zero. */
#include <stdarg.h>
long total(int count,...){va_list ap;long sum=0;int i;va_start(ap,count);for(i=0;i<count;i++)sum+=va_arg(ap,int);va_end(ap);return sum;}
int main(void)
{
    return total(9,14,21,28,35,42,49,56,63,70)!=378L;
}
