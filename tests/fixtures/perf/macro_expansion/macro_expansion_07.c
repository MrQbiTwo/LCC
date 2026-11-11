/* Token pasting, conditional directives and rescan growth; scale point 7. Expected process status: zero. */
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
#define M24 25
#define M25 26
#define M26 27
#define M27 28
#define M28 29
#define M29 30
#define M30 31
#define M31 32
#define M32 33
#define M33 34
#define M34 35
#define M35 36
#define M36 37
#define M37 38
#define M38 39
#define M39 40
#define M40 41
#define M41 42
#define M42 43
#define M43 44
#define M44 45
#define M45 46
#define M46 47
#define M47 48
#define M48 49
#define M49 50
#define M50 51
#define M51 52
#define M52 53
#define M53 54
#define M54 55
#define M55 56
#define M56 57
#define M57 58
#define M58 59
#define M59 60
#define M60 61
#define M61 62
#define M62 63
#define M63 64
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
sum+=VALUE(JOIN(M,24));
sum+=VALUE(JOIN(M,25));
sum+=VALUE(JOIN(M,26));
sum+=VALUE(JOIN(M,27));
sum+=VALUE(JOIN(M,28));
sum+=VALUE(JOIN(M,29));
sum+=VALUE(JOIN(M,30));
sum+=VALUE(JOIN(M,31));
sum+=VALUE(JOIN(M,32));
sum+=VALUE(JOIN(M,33));
sum+=VALUE(JOIN(M,34));
sum+=VALUE(JOIN(M,35));
sum+=VALUE(JOIN(M,36));
sum+=VALUE(JOIN(M,37));
sum+=VALUE(JOIN(M,38));
sum+=VALUE(JOIN(M,39));
sum+=VALUE(JOIN(M,40));
sum+=VALUE(JOIN(M,41));
sum+=VALUE(JOIN(M,42));
sum+=VALUE(JOIN(M,43));
sum+=VALUE(JOIN(M,44));
sum+=VALUE(JOIN(M,45));
sum+=VALUE(JOIN(M,46));
sum+=VALUE(JOIN(M,47));
sum+=VALUE(JOIN(M,48));
sum+=VALUE(JOIN(M,49));
sum+=VALUE(JOIN(M,50));
sum+=VALUE(JOIN(M,51));
sum+=VALUE(JOIN(M,52));
sum+=VALUE(JOIN(M,53));
sum+=VALUE(JOIN(M,54));
sum+=VALUE(JOIN(M,55));
sum+=VALUE(JOIN(M,56));
sum+=VALUE(JOIN(M,57));
sum+=VALUE(JOIN(M,58));
sum+=VALUE(JOIN(M,59));
sum+=VALUE(JOIN(M,60));
sum+=VALUE(JOIN(M,61));
sum+=VALUE(JOIN(M,62));
sum+=VALUE(JOIN(M,63));
return sum!=6304U;}
