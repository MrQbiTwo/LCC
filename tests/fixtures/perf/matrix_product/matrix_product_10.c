/* Two-dimensional matrix multiply and reduction; scale point 10. Expected process status: zero. */
int main(void)
{
    int a[11][11],b[11][11],c[11][11];int i,j,k,sum=0;
    for(i=0;i<11;i++)for(j=0;j<11;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<11;i++)for(j=0;j<11;j++){for(k=0;k<11;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=260876;
}
