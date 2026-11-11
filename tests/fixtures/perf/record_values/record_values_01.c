/* Structure array storage and by-value calls/returns; scale point 1. Expected process status: zero. */
struct Pair{long a,b;};struct Pair step(struct Pair p){p.a+=3;p.b+=4;return p;}
int main(void)
{
    struct Pair a[2],p;int i;long sum=0;
    for(i=0;i<2;i++){p.a=i*2;p.b=i*3;a[i]=step(p);sum+=a[i].a+a[i].b;}
    return sum!=19L;
}
