//>> СИНТАКСИТС
//<< простое обьявление
func add(){
}

//>> ПАРАМЕТРЫ ФУНКЦИИ ПО ЗНАЧЕНИЮ
//<< функция с параметрами по значению
// при передачи параметров в функцию по значению
// для каждой переменной параметра создается копия 
// и именно на используется внутри
// изменение значени передаваемой переменной из тела функции не возможно
// если оно передается по значению 
func add(x int, y int){
}

//<< повторяющийся тип параметров
func add(x, y int, a, b, c float32){
}

//<< неопределенное количество параметров
// в го можно передавать неограниченное количество параметров
func add(numbers ...int){
    var sum = 0
    for _, number := range numbers{
        sum += number
    }
   fmt.Println("sum = ", sum)
}
// передача их 
add(1, 2, 3)  
add(1, 2, 3, 4)   
add(5, 6, 7, 2, 3)  
digits:=[]int{2.3.5.6}
add(digits...)

// можно и явно указать некоторые из них
func add(startVal int, numbers ...int){
    for _, number := range numbers{
        startVal += number
    }
   fmt.Println("sum = ", startVal)
}

//<< пропуск параметров
// нужно когда реализуешь интерфейс
// но в твоей реализации параметр не нужен
// хотя ты обязане его указать
func printPrice(product string, price, _ float64) {
}

//>> ВОЗВРАЩАЕМЫЕ ЗНАЧЕНИЯ
//<< возврат одного значения
func add(x, y int) int {
    return x + y
}

//<< возврат именованного значения
// здесь переменная z уже обьявлена в обьявлении функции
// по этому ей я могу указать любое значение
// и могу не указывать в return
func add(x, y int) (z int) {
    z = x + y
    return
}

//<< возврат нескольких значений
func add(x, y int, firstName, lastName string) (int, string) {
    var z int = x + y
    var fullName = firstName + " " + lastName
    return z, fullName
}
// получение их из функции
var age, name = add(4, 5, "Tom", "Simpson")

//<< можно использовать и именованные результаты
func add(x, y int, firstName, lastName string) (z int, fullName string) {
    z = x + y
    fullName = firstName + " " + lastName
    return
}

//>> ПЕРЕДАЧА ПАРАМЕТРОВ ПО УКАЗАТЕЛЮ
// если внутри функции я хочу менять передаваемое значение
// я могу передать его в функцию по указателю
// в этом случае в функции создасться копия указателя
// ног она так же будет ссылаться на адрес где лежит значение
// по этому его можно менять
func getPointer(first *int) {
}

// передача указателя
val:=5;
getPointer(&val)
