/* Initialized arrays, reductions and maximum search; scale point 5. Expected process status: zero. */
int main(void)
{
    int a[8]={20,37,54,71,88,4,21,38};int i,sum=0,max=a[0];
    for(i=0;i<8;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=333||max!=88;
}
