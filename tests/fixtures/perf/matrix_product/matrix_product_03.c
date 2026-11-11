/* Two-dimensional matrix multiply and reduction; scale point 3. Expected process status: zero. */
int main(void)
{
    int a[4][4],b[4][4],c[4][4];int i,j,k,sum=0;
    for(i=0;i<4;i++)for(j=0;j<4;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<4;i++)for(j=0;j<4;j++){for(k=0;k<4;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=1568;
}
