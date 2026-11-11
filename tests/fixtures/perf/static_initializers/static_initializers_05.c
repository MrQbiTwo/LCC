/* Large constant arrays and symbolic data addresses; scale point 5. Expected process status: zero. */
unsigned values[64]={4,17,30,43,56,69,82,95,108,121,134,147,160,173,186,199,212,225,238,251,264,277,290,303,316,329,342,355,368,381,394,407,420,433,446,459,472,485,498,511,524,537,550,563,576,589,602,615,628,641,654,667,680,693,706,719,732,745,758,771,784,797,810,823};unsigned *middle=values+32;
int main(void)
{
    unsigned sum=0;int i;for(i=0;i<64;i++)sum+=values[i];
    return sum!=26464U||*middle!=420U;
}
