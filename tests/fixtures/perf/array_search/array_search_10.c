/* Initialized arrays, reductions and maximum search; scale point 10. Expected process status: zero. */
int main(void)
{
    int a[48]={45,62,79,96,12,29,46,63,80,97,13,30,47,64,81,98,14,31,48,65,82,99,15,32,49,66,83,100,16,33,50,67,84,0,17,34,51,68,85,1,18,35,52,69,86,2,19,36};int i,sum=0,max=a[0];
    for(i=0;i<48;i++){sum+=a[i];if(a[i]>max)max=a[i];}
    return sum!=2449||max!=100;
}
