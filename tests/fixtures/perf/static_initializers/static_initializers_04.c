/* Large constant arrays and symbolic data addresses; scale point 4. Expected process status: zero. */
unsigned values[48]={3,16,29,42,55,68,81,94,107,120,133,146,159,172,185,198,211,224,237,250,263,276,289,302,315,328,341,354,367,380,393,406,419,432,445,458,471,484,497,510,523,536,549,562,575,588,601,614};unsigned *middle=values+24;
int main(void)
{
    unsigned sum=0;int i;for(i=0;i<48;i++)sum+=values[i];
    return sum!=14808U||*middle!=315U;
}
