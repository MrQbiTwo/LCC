/* Static callback tables and indirect calls; scale point 10. Expected process status: zero. */
int add(int x){return x+3;}int twice(int x){return x*2;}int flip(int x){return x^7;}
int (*table[3])(int)={add,twice,flip};
int main(void)
{
    int i,sum=0;for(i=0;i<48;i++)sum+=table[i%3](i);
    return sum!=1552;
}
