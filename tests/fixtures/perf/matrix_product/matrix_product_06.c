/* Two-dimensional matrix multiply and reduction; scale point 6. Expected process status: zero. */
int main(void)
{
    int a[7][7],b[7][7],c[7][7];int i,j,k,sum=0;
    for(i=0;i<7;i++)for(j=0;j<7;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<7;i++)for(j=0;j<7;j++){for(k=0;k<7;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=26754;
}
