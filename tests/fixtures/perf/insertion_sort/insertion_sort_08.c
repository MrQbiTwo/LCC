/* In-place insertion sorting and weighted checksums; scale point 8. Expected process status: zero. */
int main(void)
{
    int a[24]={77,114,151,188,225,11,48,85,122,159,196,233,19,56,93,130,167,204,241,27,64,101,138,175};int i,j,key,sum=0;
    for(i=1;i<24;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<24;i++)sum+=(i+1)*a[i];
    return sum!=49059;
}
