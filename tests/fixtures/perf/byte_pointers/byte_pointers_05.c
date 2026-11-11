/* String traversal, byte promotion and pointer differences; scale point 5. Expected process status: zero. */
int main(void)
{
    const char *start="ELSZGNUB",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=618U||p-start!=8;
}
