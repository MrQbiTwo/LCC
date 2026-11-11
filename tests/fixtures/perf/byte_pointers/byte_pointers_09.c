/* String traversal, byte promotion and pointer differences; scale point 9. Expected process status: zero. */
int main(void)
{
    const char *start="IPWDKRYFMTAHOVCJQXELSZGNUBIPWDKR",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=2480U||p-start!=32;
}
