/* Initialized arrays, reductions and maximum search; scale point 6. Expected process status: zero. */
int main(void)
{
    int a[12]={25,42,59,76,93,9,26,43,60,77,94,10};int i,sum=0,max=a[0];
    for(i=0;i<12;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=614||max!=94;
}
