/* String traversal, byte promotion and pointer differences; scale point 2. Expected process status: zero. */
int main(void)
{
    const char *start="BIP",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=219U||p-start!=3;
}
