/* Large constant arrays and symbolic data addresses; scale point 1. Expected process status: zero. */
unsigned values[16]={0,13,26,39,52,65,78,91,104,117,130,143,156,169,182,195};unsigned *middle=values+8;
int main(void)
{
    unsigned sum=0;int i;for(i=0;i<16;i++)sum+=values[i];
    return sum!=1560U||*middle!=104U;
}
