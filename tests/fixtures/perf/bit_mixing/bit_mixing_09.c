/* Shift and XOR mixing with exact unsigned semantics; scale point 9. Expected process status: zero. */
int main(void)
{
    unsigned x=139U;int i;
    for(i=0;i<32;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=3192800458U;
}
