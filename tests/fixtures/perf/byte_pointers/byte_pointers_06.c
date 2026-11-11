/* String traversal, byte promotion and pointer differences; scale point 6. Expected process status: zero. */
int main(void)
{
    const char *start="FMTAHOVCJQXE",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=912U||p-start!=12;
}
