/* Union representation views and 64-bit shifts; scale point 10. Expected process status: zero. */
union Value{unsigned long bits;struct{unsigned int low,high;} word;};
int main(void)
{
    union Value a[48];int i;unsigned long sum=0;
    for(i=0;i<48;i++){a[i].bits=((unsigned long)(i+2)<<32)+(unsigned)(i*9+7);sum+=a[i].word.low;}
    return sum!=10488UL||a[47].word.high!=49U;
}
