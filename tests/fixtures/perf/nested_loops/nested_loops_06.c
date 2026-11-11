/* Nested loops with modulo and continue; scale point 6. Expected process status: zero. */
int main(void)
{
    int i,j,sum=0;
    for(i=0;i<12;i++) for(j=0;j<12;j++){if((i+j)%3==0)continue;sum+=(i*3+j)%17;}
    return sum!=752;
}
