/* Token pasting, conditional directives and rescan growth; scale point 4. Expected process status: zero. */
#define JOIN(a,b) a##b
#define VALUE(x) ((x)*3+1)
#if 3*7!=21
#error arithmetic
#endif
#define M0 1
#define M1 2
#define M2 3
#define M3 4
#define M4 5
#define M5 6
#define M6 7
#define M7 8
#define M8 9
#define M9 10
#define M10 11
#define M11 12
#define M12 13
#define M13 14
#define M14 15
#define M15 16
#define M16 17
#define M17 18
#define M18 19
#define M19 20
#define M20 21
#define M21 22
#define M22 23
#define M23 24
int main(void){unsigned sum=0;
sum+=VALUE(JOIN(M,0));
sum+=VALUE(JOIN(M,1));
sum+=VALUE(JOIN(M,2));
sum+=VALUE(JOIN(M,3));
sum+=VALUE(JOIN(M,4));
sum+=VALUE(JOIN(M,5));
sum+=VALUE(JOIN(M,6));
sum+=VALUE(JOIN(M,7));
sum+=VALUE(JOIN(M,8));
sum+=VALUE(JOIN(M,9));
sum+=VALUE(JOIN(M,10));
sum+=VALUE(JOIN(M,11));
sum+=VALUE(JOIN(M,12));
sum+=VALUE(JOIN(M,13));
sum+=VALUE(JOIN(M,14));
sum+=VALUE(JOIN(M,15));
sum+=VALUE(JOIN(M,16));
sum+=VALUE(JOIN(M,17));
sum+=VALUE(JOIN(M,18));
sum+=VALUE(JOIN(M,19));
sum+=VALUE(JOIN(M,20));
sum+=VALUE(JOIN(M,21));
sum+=VALUE(JOIN(M,22));
sum+=VALUE(JOIN(M,23));
return sum!=924U;}
