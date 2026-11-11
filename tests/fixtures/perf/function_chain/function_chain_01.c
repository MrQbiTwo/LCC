/* Increasing named-function counts and linked calls; scale point 1. Expected process status: zero. */
unsigned f0(unsigned x){unsigned y=x*33U+1U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
int main(void){unsigned x=3U;
x=f0(x);
return x!=500U;}
