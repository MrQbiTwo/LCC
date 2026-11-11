/* Nested loops with modulo and continue; scale point 4. Expected process status: zero. */
int main(void)
{
    int i,j,sum=0;
    for(i=0;i<6;i++) for(j=0;j<6;j++){if((i+j)%3==0)continue;sum+=(i*3+j)%17;}
    return sum!=189;
}
