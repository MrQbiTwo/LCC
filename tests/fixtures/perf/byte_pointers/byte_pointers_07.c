/* String traversal, byte promotion and pointer differences; scale point 7. Expected process status: zero. */
int main(void)
{
    const char *start="GNUBIPWDKRYFMTAH",*p=start;unsigned sum=0;
    while(*p)sum+=(unsigned char)*p++;
    return sum!=1222U||p-start!=16;
}
