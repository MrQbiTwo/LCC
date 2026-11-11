/* String traversal, byte promotion and pointer differences; scale point 8. Expected process status: zero. */
int main(void)
{
    const char *start="HOVCJQXELSZGNUBIPWDKRYFM",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=1866U||p-start!=24;
}
