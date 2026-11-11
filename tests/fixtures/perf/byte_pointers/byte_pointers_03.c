/* String traversal, byte promotion and pointer differences; scale point 3. Expected process status: zero. */
int main(void)
{
    const char *start="CJQX",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=310U||p-start!=4;
}
