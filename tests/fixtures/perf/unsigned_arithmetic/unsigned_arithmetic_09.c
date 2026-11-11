/* Wrapping unsigned multiply/add recurrences; scale point 9. Expected process status: zero. */
int main(void)
{
    unsigned x=139U,i;
    for(i=0;i<32;i++) x=x*1664525U+i+1013904223U;
    return x!=414154907U;
}
