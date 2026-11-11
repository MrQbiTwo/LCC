/* Initialized arrays, reductions and maximum search; scale point 8. Expected process status: zero. */
int main(void)
{
    int a[24]={35,52,69,86,2,19,36,53,70,87,3,20,37,54,71,88,4,21,38,55,72,89,5,22};int i,sum=0,max=a[0];
    for(i=0;i<24;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=1088||max!=89;
}
