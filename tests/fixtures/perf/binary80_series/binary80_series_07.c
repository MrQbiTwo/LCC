/* Exact binary80 arithmetic and ST0/stack ABI boundaries; scale point 7. Expected process status: zero. */
long double step(long double x){return x*0.25L;}
int main(void)
{
    int i;long double sum=0.0L;for(i=1;i<=16;i++)sum+=step((long double)i);
    return sum*4.0L!=136.0L;
}
