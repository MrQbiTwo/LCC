/* String traversal, byte promotion and pointer differences; scale point 1. Expected process status: zero. */
int main(void)
{
    const char *start="AH",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=137U||p-start!=2;
}
