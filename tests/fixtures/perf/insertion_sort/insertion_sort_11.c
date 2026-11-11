/* In-place insertion sorting and weighted checksums; scale point 11. Expected process status: zero. */
int main(void)
{
    int a[64]={110,147,184,221,7,44,81,118,155,192,229,15,52,89,126,163,200,237,23,60,97,134,171,208,245,31,68,105,142,179,216,2,39,76,113,150,187,224,10,47,84,121,158,195,232,18,55,92,129,166,203,240,26,63,100,137,174,211,248,34,71,108,145,182};int i,j,key,sum=0;
    for(i=1;i<64;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<64;i++)sum+=(i+1)*a[i];
    return sum!=346973;
}
