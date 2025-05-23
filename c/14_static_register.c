//>> СТАТИЧЕСКИ ПЕРЕМЕННЫЕ
// сколько бы не вызывали функцию,
// она создастся только один раз
// и если она изменяется,
// то ее значение сохраняется при каждом вызове

void display()
{
    // создание
    static int i = 0;
    i++;
    printf("i=%d \n", i);
}

int main(void)
{
    display(); // 1
    display(); // 2
    display(); // 3
    return 0;
}

//>> РЕГИСТРОВЫЕ ПЕРЕМЕННЫЕ
// это прееменные которые будут часто использованны в программе
// по этому ее лучше положить в регистр процессора
register int x = 8;

// в функциях так
void display(register int a)
{
    printf("a=%d \n", a);
}