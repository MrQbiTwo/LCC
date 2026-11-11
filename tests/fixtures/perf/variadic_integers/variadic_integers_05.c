/* Register and stack variable-argument traversal; scale point 5. Expected process status: zero. */
#include <stdarg.h>
long total(int count,...){va_list ap;long sum=0;int i;va_start(ap,count);for(i=0;i<count;i++)sum+=va_arg(ap,int);va_end(ap);return sum;}
int main(void)
{
    return total(6,11,18,25,32,39,46)!=171L;
}
