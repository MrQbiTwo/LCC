/* Large constant arrays and symbolic data addresses; scale point 3. Expected process status: zero. */
unsigned values[32]={2,15,28,41,54,67,80,93,106,119,132,145,158,171,184,197,210,223,236,249,262,275,288,301,314,327,340,353,366,379,392,405};unsigned *middle=values+16;
int main(void)
{
    unsigned sum=0;int i;for(i=0;i<32;i++)sum+=values[i];
    return sum!=6512U||*middle!=210U;
}
