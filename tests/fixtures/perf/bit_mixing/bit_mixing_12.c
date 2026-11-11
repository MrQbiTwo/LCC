/* Shift and XOR mixing with exact unsigned semantics; scale point 12. Expected process status: zero. */
int main(void)
{
    unsigned x=190U;int i;
    for(i=0;i<96;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=796196545U;
}
