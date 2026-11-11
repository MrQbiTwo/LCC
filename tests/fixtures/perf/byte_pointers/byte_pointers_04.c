/* String traversal, byte promotion and pointer differences; scale point 4. Expected process status: zero. */
int main(void)
{
    const char *start="DKRYFM",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=461U||p-start!=6;
}
