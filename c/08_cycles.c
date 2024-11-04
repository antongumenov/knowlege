//_ FOR
for (int i = 0; i < 9; i++)
{
    printf("%d * %d = %d \n", i, i, i * i);
}

// или так
int i = 1;
for (; i < 9;)
{
    printf("%d * %d = %d \n", i, i, i * i);
    i++;
}

//_ DO WHILE
int i = 6;
do
{
    printf("%d", i);
    i--;
} while (i > 0);

//_ WHILE
int i = 6;
while (i > 0)
{
    printf("%d \n", i);
    i--;
}

//_ ОПЕРАТОРЫ
// continue - на следующий цикл
// break - выйти из цикла