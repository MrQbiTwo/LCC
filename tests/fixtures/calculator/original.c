#include <stdio.h>

int main(void) {
int choice; 
double num1, num2, result; 
int int_result; 
char continue_calc; 

printf("=== Calculator ===\n\n"); 

do {
printf("Choose an operation:\n"); printf("1 - Addition (+)\n"); 
printf("2 - Subtraction (-)\n"); 
printf("3 - Multiplication (*)\n"); 
printf("4 - Division (/)\n"); 
printf("5 - Modulo (%%)\n"); 
printf("6 - Power\n"); 
printf("Your choice: "); 

if (scanf("%d", &choice) != 1) { 
printf("Input error! Enter a number.\n"); 
while (getchar() != '\n'); 
continue; 
} 

printf("Enter first number: "); 
if (scanf("%lf", &num1) != 1) { 
printf("Input error!\n"); 
while (getchar() != '\n'); 
continue; 
} 

printf("Enter second number: "); 
if (scanf("%lf", &num2) != 1) { 
printf("Input error!\n"); 
while (getchar() != '\n'); 
continue; 
} 

switch (choice) { 
case 1: 
result = num1 + num2; 
printf("%.2lf + %.2lf = %.2lf\n", num1, num2, result); 
break; 

case 2: 
result = num1 - num2; 
printf("%.2lf - %.2lf = %.2lf\n", num1, num2, result); 
break; 

case 3: 
result = num1 * num2; 
printf("%.2lf * %.2lf = %.2lf\n", num1, num2, result); 
break; 

case 4: 
if (num2 == 0.0) { 
printf("Error: division by zero!\n"); 
} else { 
result = num1 / num2; 
printf("%.2lf / %.2lf = %.2lf\n", num1, num2, result); 
} 
break; 

case 5: { 
int a, b; 
if (num1 != (int)num1 || num2 != (int)num2) { 
printf("Modulo works only with integers!\n"); 
} else if ((int)num2 == 0) { 
printf("Error: division by zero!\n"); 
} else { 
a = (int)num1; 
b = (int)num2; 
int_result = a % b; 
printf("%d %% %d = %d\n", a, b, int_result); 
} 
break; 
} 

case 6: { 
int i; 
result = 1.0; 
if (num2 < 0 || num2 != (int)num2) { 
printf("Exponent must be a non-negative integer!\n"); 
} else { 
for (i = 0; i < (int)num2; i++) { 
result *= num1; 
} 
printf("%.2lf ^ %d = %.2lf\n", num1, (int)num2, result); 
} 
break; 
} 

default: 
printf("Invalid operation choice!\n"); 
break; 
} 

printf("\nContinue? (y/n): "); 
while (getchar() != '\n'); 
continue_calc = getchar(); 
printf("\n"); 

} while (continue_calc == 'y' || continue_calc == 'Y'); 

printf("Goodbye!\n"); 
return 0;
}