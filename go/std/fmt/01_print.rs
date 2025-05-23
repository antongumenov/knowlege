//>> НАПИСАНИЕ СТРОК
//<< Print(...vals)
// выводит значения в констоль через пробел если они не являются строками
fmt.Print("мама", "папа", 17, 20) // мамапапа17 20

//<< PrintLn(...vals)
// выводит значения в констоль через пробел
fmt.Println("мама", "папа", 17, 20) // мама папа 17 20

//<< Fprint(writer,...vals)
// выводит значения в writer через пробел если они не являются строками, дописывая а не переписывая
file, _ := os.Create("1.txt")
fmt.Fprint(file, "мама", "папа", 17, 20) // мамапапа17 20

//<< PrintLn(...vals)
// выводит значения в writer через пробел, дописывая а не переписывая
fmt.Fprintln(file, "мама", "папа", 17, 20) // мама папа 17 20

//>> ФОРМАТИРОВАННЫЙ ВВОД
//<< Sprintf(t,...vals)
// возвращает строку отформатированную по шаблону
str := fmt.Sprintf("мама любит %s", "папу") // мамапапа17 20
fmt.Println(str)                            // мама любит папу

//<< Printf(t,...vals)
// выводит строку отформатированную по шаблону в консоль
fmt.Printf("мама любит %s", "папу") // мама любит папу

//<< Fprintf(writer, t,...vals)
// выводит строку отфарматированную по шаблону в writer
fmt.Fprintf(file, "мама любит %s", "папу") // мамапапа17 20

//<< Errorf(writer, t,...vals)
// создает ошибку используя шаблон
err := fmt.Errorf("мама любит %s", "папу") // мамапапа17 20
fmt.Println(err)

//>> ГЛАГОЛЫ ФОРМАТИРОВАНИЯ
// %t: для вывода значений типа boolean (true или false)
// %b: для вывода целых чисел в двоичной системе
// %c: для вывода символов, представленных числовым кодом
// %d: для вывода целых чисел в десятичной системе
// %o: для вывода целых чисел в восьмеричной системе
// %q: для вывода символов в одинарных кавычках
// %x: для вывода целых чисел в шестнадцатиричной системе, буквенные символы числа имеют нижний регистр a-f
// %X: для вывода целых чисел в шестнадцатиричной системе, буквенные символы числа имеют верхний регистр A-F
// %U: для вывода символов в формате кодов Unicode, например, U+1234
// %e: для вывода чисел с плавающей точкой в экспоненциальном представлении, например, -1.234456e+78
// %E: для вывода чисел с плавающей точкой в экспоненциальном представлении, например, -1.234456E+78
// %f: для вывода чисел с плавающей точкой, например, 123.456 
// %F: то же самое, что и %f
// %g: для длинных чисел с плаващей точкой используется %e, для других - %f
// %G: для длинных чисел с плаващей точкой используется %E, для других - %F
// %s: для вывода строки
// %p: для вывода значения указателя - адреса в шестнадцатеричном представлении
// %T: тип переменной

//<< использование глаголов форматирования строк
name := "Kayak"
fmt.Printf("String: %s\n", name)               // String: Kayak
fmt.Printf("Character: %c\n", []rune(name)[0]) // Character: K
fmt.Printf("Unicode: %U\n", []rune(name)[0])   // Unicode: U+004B

//<< глаголы bool
name := "Kayak"
fmt.Printf("Bool: %t\n", len(name) > 1)   // Bool: true
fmt.Printf("Bool: %t\n", len(name) > 100) // Bool: false

//<< глагол указателя
name := "Kayak"
Printfln("Pointer: %p", &name)  // Pointer: 0xc00004a240

//<< доп символы
number := 279.00
fmt.Printf("Sign: >>%8.2f<<\n", number)                // Sign: >>  279.00<< - вывести 8 цифр, две после запятой
fmt.Printf("Sign: >>%+.2f<<\n", number)                // Sign: >>+279.00<< - показать знак числа
fmt.Printf("Zeros for Padding: >>%010.2f<<\n", number) // Zeros for Padding: >>0000279.00<< - вывести 10 цифр, остальное заполнить нулями
fmt.Printf("Right Padding: >>%-8.2f<<\n", number)      // Right Padding: >>279.00  << - вывести 8 цифр но пробелы поставить назад

//>> ИНТЕРФЕЙС STRINGER
// го по умолчанию определяет как будет выводится труктура
// но можно изменить вывод по умолчанию своим вариантом
// имплементировав интерфейс Stringer
type Product struct {
	Name, Category string
	Price float64
}
	
func (p Product) String() string {
	return fmt.Sprintf("Product: %v, Price: $%4.2f", p.Name, p.Price)
}