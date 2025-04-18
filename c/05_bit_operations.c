//>> ИНТЕРЕСНЫЕ ОПЕРАЦИИ
// префиксный инкремент, вначале сложит потом присвоит
// декремент тоже самое
int a = 8;
int b = ++a;
printf("a = %d \n", a); // 9
printf("b = %d", b);    // 9

// постфиксный инкремент, вначале присвоит, потом сложит
// декремент то же самое
int a = 8;
int b = a++;
printf("a = %d \n", a); // 9
printf("b = %d", b);    // 8

// тройной
int c = a++ + b;
// нужно рассматривать как
int c = (a++) + b;

//>> СДВИГ
int a = 2 << 2;  // 10  на два разрядов влево = 1000 - 8
int b = 16 >> 3; // 10000 на три разряда вправо = 10 - 2

//>> ПОРАЗРЯДНЫЕ ОПЕРАЦИИ

int a = 5 | 2; // 101 | 010 = 111  - 7 - вернет 1 если хотя бы 1 равен 1
int b = 6 & 2; // 110 & 010 = 10  - 2 - вернет один если оба равны 1
int c = 5 ^ 2; // 101 ^ 010 = 111 - 7 - вернет один если только 1 равен 1 а втрой 0

int f = 12; // 00001100 - меняет 1 на ноль, ноль на 1
int d = ~f; // 11110011   или -13

//>> ОТРИЦАТЕЛЬНЫЙ ЧИСЛА
// в двоичной 0 - это 00000000 - положительное число
// 1 - это 00000001
// отрицательные инвертированы, первый разряд это знак числа
// -1 - это 11111111
// -13 = 11110011

// чтобы отрицательное число перевести в положительное
// нужно число инвертировать и прибавить 1
int x = 12;
int y = ~x;
y += 1;
printf("y = %d \n", y); // y=-12

//>> ПРИСВАИВАНИЕ
// можно сделать так, все будут равны 41
int a, b, c;
a = b = c = 34 + 7;

//<< операции дейсвтие плюс присваивание
// +    эквивалент  A = A + B
// -    эквивалент  A = A - B
// /    эквивалент  A = A / B
// *    эквивалент  A = A * B
// %    эквивалент  A = A % B
// <<   эквивалент  A = A << B
// >>   эквивалент  A = A >> B
// &    эквивалент  A = A & B
// |    эквивалент  A = A | B
// ^    эквивалент  A = A ^ B