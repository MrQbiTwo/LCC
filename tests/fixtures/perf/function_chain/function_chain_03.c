/* Increasing named-function counts and linked calls; scale point 3. Expected process status: zero. */
unsigned f0(unsigned x){unsigned y=x*33U+1U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f1(unsigned x){unsigned y=x*33U+2U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f2(unsigned x){unsigned y=x*33U+3U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f3(unsigned x){unsigned y=x*33U+4U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
int main(void){unsigned x=37U;
x=f0(x);
x=f1(x);
x=f2(x);
x=f3(x);
return x!=1024772658U;}
