/* In-place insertion sorting and weighted checksums; scale point 5. Expected process status: zero. */
int main(void)
{
    int a[8]={44,81,118,155,192,229,15,52};int i,j,key,sum=0;
    for(i=1;i<8;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<8;i++)sum+=(i+1)*a[i];
    return sum!=5279;
}
