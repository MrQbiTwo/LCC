/* Initialized arrays, reductions and maximum search; scale point 4. Expected process status: zero. */
int main(void)
{
    int a[6]={15,32,49,66,83,100};int i,sum=0,max=a[0];
    for(i=0;i<6;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=345||max!=100;
}
