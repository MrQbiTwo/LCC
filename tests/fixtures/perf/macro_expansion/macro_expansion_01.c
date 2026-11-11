/* Token pasting, conditional directives and rescan growth; scale point 1. Expected process status: zero. */
#define JOIN(a,b) a##b
#define VALUE(x) ((x)*3+1)
#if 3*7!=21
#error arithmetic
#endif
#define M0 1
#define M1 2
#define M2 3
#define M3 4
int main(void){unsigned sum=0;
sum+=VALUE(JOIN(M,0));
sum+=VALUE(JOIN(M,1));
sum+=VALUE(JOIN(M,2));
sum+=VALUE(JOIN(M,3));
return sum!=34U;}
