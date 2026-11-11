/* String traversal, byte promotion and pointer differences; scale point 10. Expected process status: zero. */
int main(void)
{
    const char *start="JQXELSZGNUBIPWDKRYFMTAHOVCJQXELSZGNUBIPWDKRYFMTA",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=3726U||p-start!=48;
}
