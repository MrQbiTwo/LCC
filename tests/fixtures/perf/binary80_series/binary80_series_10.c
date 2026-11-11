/* Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 10. Expected process status: zero. */
long double step(long double x){return x*0.25L;}
int main(void)
{
    int i;long double sum=0.0L;for(i=1;i<=48;i++)sum+=step((long double)i);
    return sum*4.0L!=1176.0L;
}
