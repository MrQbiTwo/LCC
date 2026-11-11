/* Initialized arrays, reductions and maximum search; scale point 11. Expected process status: zero. */
int main(void)
{
    int a[64]={50,67,84,0,17,34,51,68,85,1,18,35,52,69,86,2,19,36,53,70,87,3,20,37,54,71,88,4,21,38,55,72,89,5,22,39,56,73,90,6,23,40,57,74,91,7,24,41,58,75,92,8,25,42,59,76,93,9,26,43,60,77,94,10};int i,sum=0,max=a[0];
    for(i=0;i<64;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=3031||max!=94;
}
