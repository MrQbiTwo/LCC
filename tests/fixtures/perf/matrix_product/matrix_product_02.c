/* Two-dimensional matrix multiply and reduction; scale point 2. Expected process status: zero. */
int main(void)
{
    int a[3][3],b[3][3],c[3][3];int i,j,k,sum=0;
    for(i=0;i<3;i++)for(j=0;j<3;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<3;i++)for(j=0;j<3;j++){for(k=0;k<3;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=360;
}
