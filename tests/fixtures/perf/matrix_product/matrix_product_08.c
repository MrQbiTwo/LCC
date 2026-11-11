/* Two-dimensional matrix multiply and reduction; scale point 8. Expected process status: zero. */
int main(void)
{
    int a[9][9],b[9][9],c[9][9];int i,j,k,sum=0;
    for(i=0;i<9;i++)for(j=0;j<9;j++){a[i][j]=i+j+1;b[i][j]=i*2+j+1;c[i][j]=0;}
    for(i=0;i<9;i++)for(j=0;j<9;j++){for(k=0;k<9;k++)c[i][j]+=a[i][k]*b[k][j];sum+=c[i][j];}
    return sum!=95013;
}
