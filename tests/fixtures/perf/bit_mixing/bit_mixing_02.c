/* Shift and XOR mixing with exact unsigned semantics; scale point 2. Expected process status: zero. */
int main(void)
{
    unsigned x=20U;int i;
    for(i=0;i<3;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=2599943120U;
}
