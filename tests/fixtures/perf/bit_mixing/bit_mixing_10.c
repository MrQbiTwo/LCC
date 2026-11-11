/* Shift and XOR mixing with exact unsigned semantics; scale point 10. Expected process status: zero. */
int main(void)
{
    unsigned x=156U;int i;
    for(i=0;i<48;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=3613856561U;
}
