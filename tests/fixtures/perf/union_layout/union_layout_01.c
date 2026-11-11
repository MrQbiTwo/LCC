/* Union representation views and 64-bit shifts; scale point 1. Expected process status: zero. */
union Value{unsigned long bits;struct{unsigned int low,high;} word;};
int main(void)
{
    union Value a[2];int i;unsigned long sum=0;
    for(i=0;i<2;i++){a[i].bits=((unsigned long)(i+2)<<32)+(unsigned)(i*9+7);sum+=a[i].word.low;}
    return sum!=23UL||a[1].word.high!=3U;
}
