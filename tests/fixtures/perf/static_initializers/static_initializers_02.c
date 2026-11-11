/* Large constant arrays and symbolic data addresses; scale point 2. Expected process status: zero. */
unsigned values[24]={1,14,27,40,53,66,79,92,105,118,131,144,157,170,183,196,209,222,235,248,261,274,287,300};unsigned *middle=values+12;
int main(void)
{
    unsigned sum=0;int i;for(i=0;i<24;i++)sum+=values[i];
    return sum!=3612U||*middle!=157U;
}
