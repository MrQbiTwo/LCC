/* Recursive Fibonacci call trees; scale point 2. Expected process status: zero. */
unsigned fib(unsigned n){if(n<2)return n;return fib(n-1)+fib(n-2);}
int main(void)
{
    return fib(6)!=8U;
}
