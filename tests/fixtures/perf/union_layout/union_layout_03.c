/* Union representation views and 64-bit shifts; scale point 3. Expected process status: zero. */
union Value{unsigned long bits;struct{unsigned int low,high;} word;};
int main(void)
{
    union Value a[4];int i;unsigned long sum=0;
    for(i=0;i<4;i++){a[i].bits=((unsigned long)(i+2)<<32)+(unsigned)(i*9+7);sum+=a[i].word.low;}
    return sum!=82UL||a[3].word.high!=5U;
}
