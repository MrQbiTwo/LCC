/* Wrapping unsigned multiply/add recurrences; scale point 5. Expected process status: zero. */
int main(void)
{
    unsigned x=71U,i;
    for(i=0;i<8;i++) x=x*1664525U+i+1013904223U;
    return x!=314673387U;
}
