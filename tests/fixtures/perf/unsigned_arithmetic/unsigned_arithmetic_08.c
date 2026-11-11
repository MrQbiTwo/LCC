/* Wrapping unsigned multiply/add recurrences; scale point 8. Expected process status: zero. */
int main(void)
{
    unsigned x=122U,i;
    for(i=0;i<24;i++) x=x*1664525U+i+1013904223U;
    return x!=3084687174U;
}
