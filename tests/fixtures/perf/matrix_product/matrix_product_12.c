/* Two-dimensional matrix multiply and reduction; scale point 12. Expected process status: zero. */
int main(void)
{
    int a[13][13],b[13][13],c[13][13];int i,j,k,sum=0;
    for(i=0;i<13;i++)for(j=0;j<13;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<13;i++)for(j=0;j<13;j++){for(k=0;k<13;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=604175;
}
