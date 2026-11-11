/* Two-dimensional matrix multiply and reduction; scale point 11. Expected process status: zero. */
int main(void)
{
    int a[12][12],b[12][12],c[12][12];int i,j,k,sum=0;
    for(i=0;i<12;i++)for(j=0;j<12;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<12;i++)for(j=0;j<12;j++){for(k=0;k<12;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=404064;
}
