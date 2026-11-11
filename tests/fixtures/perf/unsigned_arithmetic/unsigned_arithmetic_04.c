/* Wrapping unsigned multiply/add recurrences; scale point 4. Expected process status: zero. */
int main(void)
{
    unsigned x=54U,i;
    for(i=0;i<6;i++) x=x*1664525U+i+1013904223U;
    return x!=2913227899U;
}
