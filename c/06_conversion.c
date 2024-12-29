//_ ПРЕОБРАЗОВАНИЕ ТИПОВ
//~ правила преобразования
// Если один из операндов имеет тип long double, то второй операнд тоже будет преобразован в тип long double
// Если предыдущий пункт не выполняется и если один из операндов имеет тип double, то второй операнд тоже будет преобразован к типу double
// Если предыдущий пункт не выполняется и если один из операндов имеет тип float, то второй операнд тоже будет преобразован к типу float
// Если предыдущий пункт не выполняется и если один из операндов имеет тип unsigned long int, то второй операнд тоже будет преобразован к типу unsigned long int
// Если предыдущий пункт не выполняется и если один из операндов имеет тип long, то второй операнд тоже будет преобразован к типу long
// Если предыдущий пункт не выполняется и если один из операндов имеет тип unsigned, то второй операнд тоже будет преобразован к типу unsigned
// Если предыдущий пункт не выполняется то оба операнда приводятся к типу int

// при сложении все пойдет норм
int number1 = 10;
double number2 = 4;
double result = number1 + number2; // 14.000000

// а вот при делении есть нюансы
int a = 10;
int b = 4;
int c = a / b;                    // 2
double d = a / b;                 // 2.00000
double e = (double)a / (double)b; // 2.50000

// преобразования символов
int number = 70;
char symbol = (char)number;
printf("symbol = %c \n", symbol);            //  F
printf("symbol (int code) = %d \n", symbol); // 70

//! АКУРАТНО С ПРИОБРАЗОВАНИЕМ, МОГУТ УТЕЧ ДАННЫЕ