/* Wrapping unsigned multiply/add recurrences; scale point 10. Expected process status: zero. */
int main(void)
{
    unsigned x=156U,i;
    for(i=0;i<48;i++) x=x*1664525U+i+1013904223U;
    return x!=2186941684U;
}
