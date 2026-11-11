/* Shift and XOR mixing with exact unsigned semantics; scale point 7. Expected process status: zero. */
int main(void)
{
    unsigned x=105U;int i;
    for(i=0;i<16;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=3047028332U;
}
