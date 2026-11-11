/* Initialized arrays, reductions and maximum search; scale point 7. Expected process status: zero. */
int main(void)
{
    int a[16]={30,47,64,81,98,14,31,48,65,82,99,15,32,49,66,83};int i,sum=0,max=a[0];
    for(i=0;i<16;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=904||max!=99;
}
