/* Shift and XOR mixing with exact unsigned semantics; scale point 1. Expected process status: zero. */
int main(void)
{
    unsigned x=3U;int i;
    for(i=0;i<2;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=201886211U;
}
