/* Wrapping unsigned multiply/add recurrences; scale point 11. Expected process status: zero. */
int main(void)
{
    unsigned x=173U,i;
    for(i=0;i<64;i++) x=x*1664525U+i+1013904223U;
    return x!=3841750733U;
}
