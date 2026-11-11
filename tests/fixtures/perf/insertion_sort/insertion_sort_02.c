/* In-place insertion sorting and weighted checksums; scale point 2. Expected process status: zero. */
int main(void)
{
    int a[3]={11,48,85};int i,j,key,sum=0;
    for(i=1;i<3;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<3;i++)sum+=(i+1)*a[i];
    return sum!=362;
}
