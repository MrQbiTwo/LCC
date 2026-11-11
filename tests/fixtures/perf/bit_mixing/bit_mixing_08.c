/* Shift and XOR mixing with exact unsigned semantics; scale point 8. Expected process status: zero. */
int main(void)
{
    unsigned x=122U;int i;
    for(i=0;i<24;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=4141199992U;
}
