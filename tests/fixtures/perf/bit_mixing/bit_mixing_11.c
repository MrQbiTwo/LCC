/* Shift and XOR mixing with exact unsigned semantics; scale point 11. Expected process status: zero. */
int main(void)
{
    unsigned x=173U;int i;
    for(i=0;i<64;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=3347381571U;
}
