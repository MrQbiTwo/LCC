/* Two-dimensional matrix multiply and reduction; scale point 7. Expected process status: zero. */
int main(void)
{
    int a[8][8],b[8][8],c[8][8];int i,j,k,sum=0;
    for(i=0;i<8;i++)for(j=0;j<8;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<8;i++)for(j=0;j<8;j++){for(k=0;k<8;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=52480;
}
