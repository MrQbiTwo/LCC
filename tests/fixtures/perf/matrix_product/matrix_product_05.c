/* Two-dimensional matrix multiply and reduction; scale point 5. Expected process status: zero. */
int main(void)
{
    int a[6][6],b[6][6],c[6][6];int i,j,k,sum=0;
    for(i=0;i<6;i++)for(j=0;j<6;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<6;i++)for(j=0;j<6;j++){for(k=0;k<6;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=12276;
}
