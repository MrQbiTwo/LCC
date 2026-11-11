/* Increasing named-function counts and linked calls; scale point 2. Expected process status: zero. */
unsigned f0(unsigned x){unsigned y=x*33U+1U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f1(unsigned x){unsigned y=x*33U+2U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
int main(void){unsigned x=20U;
x=f0(x);
x=f1(x);
return x!=20700U;}
