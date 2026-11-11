/* Two-dimensional matrix multiply and reduction; scale point 1. Expected process status: zero. */
int main(void)
{
    int a[2][2],b[2][2],c[2][2];int i,j,k,sum=0;
    for(i=0;i<2;i++)for(j=0;j<2;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<2;i++)for(j=0;j<2;j++){for(k=0;k<2;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=44;
}
