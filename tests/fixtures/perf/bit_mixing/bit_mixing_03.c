/* Shift and XOR mixing with exact unsigned semantics; scale point 3. Expected process status: zero. */
int main(void)
{
    unsigned x=37U;int i;
    for(i=0;i<4;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=3782103551U;
}
