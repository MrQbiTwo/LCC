/* Initialized arrays, reductions and maximum search; scale point 3. Expected process status: zero. */
int main(void)
{
    int a[4]={10,27,44,61};int i,sum=0,max=a[0];
    for(i=0;i<4;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=142||max!=61;
}
