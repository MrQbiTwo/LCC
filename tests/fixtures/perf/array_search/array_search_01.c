/* Initialized arrays, reductions and maximum search; scale point 1. Expected process status: zero. */
int main(void)
{
    int a[2]={0,17};int i,sum=0,max=a[0];
    for(i=0;i<2;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=17||max!=17;
}
