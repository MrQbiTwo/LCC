/* Wrapping unsigned multiply/add recurrences; scale point 7. Expected process status: zero. */
int main(void)
{
    unsigned x=105U,i;
    for(i=0;i<16;i++) x=x*1664525U+i+1013904223U;
    return x!=2718040433U;
}
