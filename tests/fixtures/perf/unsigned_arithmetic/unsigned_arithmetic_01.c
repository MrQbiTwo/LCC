/* Wrapping unsigned multiply/add recurrences; scale point 1. Expected process status: zero. */
int main(void)
{
    unsigned x=3U,i;
    for(i=0;i<2;i++) x=x*1664525U+i+1013904223U;
    return x!=2365144878U;
}
