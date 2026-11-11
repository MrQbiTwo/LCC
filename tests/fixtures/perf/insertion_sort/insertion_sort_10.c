/* In-place insertion sorting and weighted checksums; scale point 10. Expected process status: zero. */
int main(void)
{
    int a[48]={99,136,173,210,247,33,70,107,144,181,218,4,41,78,115,152,189,226,12,49,86,123,160,197,234,20,57,94,131,168,205,242,28,65,102,139,176,213,250,36,73,110,147,184,221,7,44,81};int i,j,key,sum=0;
    for(i=1;i<48;i++){key=a[i];j=i;while(j>0&&a[j-1]>key){a[j]=a[j-1];j--;}a[j]=key;}
    for(i=0;i<48;i++)sum+=(i+1)*a[i];
    return sum!=196970;
}
