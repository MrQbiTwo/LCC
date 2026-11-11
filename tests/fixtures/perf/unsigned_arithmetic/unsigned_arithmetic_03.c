/* Wrapping unsigned multiply/add recurrences; scale point 3. Expected process status: zero. */
int main(void)
{
    unsigned x=37U,i;
    for(i=0;i<4;i++) x=x*1664525U+i+1013904223U;
    return x!=553841647U;
}
