/* Two-dimensional matrix multiply and reduction; scale point 4. Expected process status: zero. */
int main(void)
{
    int a[5][5],b[5][5],c[5][5];int i,j,k,sum=0;
    for(i=0;i<5;i++)for(j=0;j<5;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<5;i++)for(j=0;j<5;j++){for(k=0;k<5;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=4875;
}
