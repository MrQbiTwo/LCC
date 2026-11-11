/* String traversal, byte promotion and pointer differences; scale point 11. Expected process status: zero. */
int main(void)
{
    const char *start="KRYFMTAHOVCJQXELSZGNUBIPWDKRYFMTAHOVCJQXELSZGNUBIPWDKRYFMTAHOVCJ",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=4950U||p-start!=64;
}
