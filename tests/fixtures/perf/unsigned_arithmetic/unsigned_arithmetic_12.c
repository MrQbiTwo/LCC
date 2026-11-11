/* Wrapping unsigned multiply/add recurrences; scale point 12. Expected process status: zero. */
int main(void)
{
    unsigned x=190U,i;
    for(i=0;i<96;i++) x=x*1664525U+i+1013904223U;
    return x!=2813247854U;
}
