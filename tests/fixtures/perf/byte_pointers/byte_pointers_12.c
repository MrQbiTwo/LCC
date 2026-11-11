/* String traversal, byte promotion and pointer differences; scale point 12. Expected process status: zero. */
int main(void)
{
    const char *start="LSZGNUBIPWDKRYFMTAHOVCJQXELSZGNUBIPWDKRYFMTAHOVCJQXELSZGNUBIPWDKRYFMTAHOVCJQXELSZGNUBIPWDKRYFMTA",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=7444U||p-start!=96;
}
