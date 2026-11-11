/* Two-dimensional matrix multiply and reduction; scale point 9. Expected process status: zero. */
int main(void)
{
    int a[10][10],b[10][10],c[10][10];int i,j,k,sum=0;
    for(i=0;i<10;i++)for(j=0;j<10;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<10;i++)for(j=0;j<10;j++){for(k=0;k<10;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=161500;
}
