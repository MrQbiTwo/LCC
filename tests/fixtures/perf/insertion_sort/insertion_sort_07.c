/* In-place insertion sorting and weighted checksums; scale point 7. Expected process status: zero. */
int main(void)
{
    int a[16]={66,103,140,177,214,0,37,74,111,148,185,222,8,45,82,119};int i,j,key,sum=0;
    for(i=1;i<16;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<16;i++)sum+=(i+1)*a[i];
    return sum!=19637;
}
