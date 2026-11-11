/* Exact binary64 arithmetic across function boundaries; scale point 10. Expected process status: zero. */
double step(double x){return x*0.5;}
int main(void)
{
    int i;double sum=0.0;for(i=1;i<=48;i++)sum+=step((double)i);
    return sum*2.0!=1176.0;
}
