/* Initialized arrays, reductions and maximum search; scale point 2. Expected process status: zero. */
int main(void)
{
    int a[3]={5,22,39};int i,sum=0,max=a[0];
    for(i=0;i<3;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=66||max!=39;
}
