/* Shift and XOR mixing with exact unsigned semantics; scale point 6. Expected process status: zero. */
int main(void)
{
    unsigned x=88U;int i;
    for(i=0;i<12;i++){x^=x<<13;x^=x>>17;x^=x<<5;}
    return x!=2724819854U;
}
