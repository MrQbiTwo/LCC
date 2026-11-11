/* Wrapping unsigned multiply/add recurrences; scale point 6. Expected process status: zero. */
int main(void)
{
    unsigned x=88U,i;
    for(i=0;i<12;i++) x=x*1664525U+i+1013904223U;
    return x!=3620074806U;
}
