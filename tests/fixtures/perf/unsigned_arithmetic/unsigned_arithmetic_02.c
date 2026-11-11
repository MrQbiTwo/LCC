/* Wrapping unsigned multiply/add recurrences; scale point 2. Expected process status: zero. */
int main(void)
{
    unsigned x=20U,i;
    for(i=0;i<3;i++) x=x*1664525U+i+1013904223U;
    return x!=2207985820U;
}
