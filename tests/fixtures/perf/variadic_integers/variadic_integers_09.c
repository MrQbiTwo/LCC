/* Register and stack variable-argument traversal; scale point 9. Expected process status: zero. */
#include <stdarg.h>
long total(int count,...){va_list ap;long sum=0;int i;va_start(ap,count);for(i=0;i<count;i++)sum+=va_arg(ap,int);va_end(ap);return sum;}
int main(void)
{
    return total(10,15,22,29,36,43,50,57,64,71,78)!=465L;
}
