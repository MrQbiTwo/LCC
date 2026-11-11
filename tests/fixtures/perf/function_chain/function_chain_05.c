/* Increasing named-function counts and linked calls; scale point 5. Expected process status: zero. */
unsigned f0(unsigned x){unsigned y=x*33U+1U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f1(unsigned x){unsigned y=x*33U+2U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f2(unsigned x){unsigned y=x*33U+3U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f3(unsigned x){unsigned y=x*33U+4U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f4(unsigned x){unsigned y=x*33U+5U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f5(unsigned x){unsigned y=x*33U+6U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f6(unsigned x){unsigned y=x*33U+7U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f7(unsigned x){unsigned y=x*33U+8U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f8(unsigned x){unsigned y=x*33U+9U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f9(unsigned x){unsigned y=x*33U+10U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f10(unsigned x){unsigned y=x*33U+11U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f11(unsigned x){unsigned y=x*33U+12U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f12(unsigned x){unsigned y=x*33U+13U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f13(unsigned x){unsigned y=x*33U+14U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f14(unsigned x){unsigned y=x*33U+15U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
unsigned f15(unsigned x){unsigned y=x*33U+16U;if(y&1U)y^=y>>3;else y+=y<<2;return y;}
int main(void){unsigned x=71U;
x=f0(x);
x=f1(x);
x=f2(x);
x=f3(x);
x=f4(x);
x=f5(x);
x=f6(x);
x=f7(x);
x=f8(x);
x=f9(x);
x=f10(x);
x=f11(x);
x=f12(x);
x=f13(x);
x=f14(x);
x=f15(x);
return x!=1881118748U;}
