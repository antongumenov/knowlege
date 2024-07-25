//_ МАССИВЫ
// набор значений одного типа с неизменяемой длинной

//~ обьявление
var names [3]string
names[0] = "Kayak"
names[1] = "Lifejacket"
names[2] = "Paddle"

//~ литеральный синтаксис
names := [3]string { "Kayak", "Lifejacket", "Paddle" }

//~ обьявление с значениями по умолчанию
// 2 и 3 значениями будут пустые строки
names := [3]string { "Kayak" }

//~ многомерный массив
var coords [3][3]int
coords[1][2] = 10

//~ автоопределение длинны массива
// компилятор сам определит длинну
names := [...]string { "Kayak", "Lifejacket", "Paddle" }

//_ СРАВНЕНИЕ МАССИВОВ
// сравнивать можно при помощи == и !==

// массивы равны если 
// в них одни и те же элементы, 
// одного и того же типа 
// и в одинаковом порядке

//_ ИТЕРАЦИЯ ПО ЭЛЕМЕНТАМ МАССИВА
names := [3]string { "Kayak", "Lifejacket", "Paddle" }

for index, value := range names {
fmt.Println("Index:", index, "Value:", value)
}
