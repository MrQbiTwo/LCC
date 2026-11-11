/* Seven-way switch dispatch and branch joins; scale point 8. Expected process status: zero. */
int main(void)
{
    int i,sum=0;
    for(i=0;i<24;i++){switch(i%7){case 0:sum+=i*3;break;case 1:sum+=i+9;break;case 2:sum+=i*2;break;case 3:sum+=17;break;case 4:sum+=i^11;break;case 5:sum+=i/2;break;default:sum+=5;}}
    return sum!=431;
}
