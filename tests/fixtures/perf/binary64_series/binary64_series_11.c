/* Exact binary64 arithmetic across function boundaries; scale point 11. Expected process status: zero. */
double step(double x){return x*0.5;}
int main(void)
{
    int i;double sum=0.0;for(i=1;i<=64;i++)sum+=step((double)i);
    return sum*2.0!=2080.0;
}
