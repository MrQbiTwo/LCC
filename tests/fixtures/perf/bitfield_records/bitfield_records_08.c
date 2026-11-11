/* Packed bit-field assignments, extraction and truncation; scale point 8. Expected process status: zero. */
struct Bits{unsigned a:3;unsigned b:5;unsigned c:10;};
int main(void)
{
    struct Bits values[24];int i;unsigned sum=0;
    for(i=0;i<24;i++){values[i].a=i;values[i].b=i*5;values[i].c=i*11;sum+=values[i].a+values[i].b+values[i].c;}
    return sum!=3476U;
}
