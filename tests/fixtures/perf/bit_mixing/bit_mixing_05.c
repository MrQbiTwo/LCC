/* Shift and XOR mixing with exact unsigned semantics; scale point 5. Expected process status: zero. */
int main(void)
{
    unsigned x=71U;int i;
    for(i=0;i<8;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=3627583310U;
}
