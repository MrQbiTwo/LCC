/* In-place insertion sorting and weighted checksums; scale point 9. Expected process status: zero. */
int main(void)
{
    int a[32]={88,125,162,199,236,22,59,96,133,170,207,244,30,67,104,141,178,215,1,38,75,112,149,186,223,9,46,83,120,157,194,231};int i,j,key,sum=0;
    for(i=1;i<32;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<32;i++)sum+=(i+1)*a[i];
    return sum!=88589;
}
