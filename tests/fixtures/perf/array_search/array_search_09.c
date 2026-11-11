/* Initialized arrays, reductions and maximum search; scale point 9. Expected process status: zero. */
int main(void)
{
    int a[32]={40,57,74,91,7,24,41,58,75,92,8,25,42,59,76,93,9,26,43,60,77,94,10,27,44,61,78,95,11,28,45,62};int i,sum=0,max=a[0];
    for(i=0;i<32;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=1632||max!=95;
}
