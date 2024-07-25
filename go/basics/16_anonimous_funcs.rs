//_ АНОНИМНЫЕ ФУНКЦИИ
// чаще всего применяются для единовременного запуска функции в горутине
go func(x, y int) int{ return x + y}(5,4)

// так же можно ее присвоить переменной
f := func(x, y int) int{ return x + y}
fmt.Println(f(3, 4))        // 7
fmt.Println(f(6, 7))        // 13

//_ АНОНИМНАЯ ФУНКЦИЯ КАК АРГУМЕНТ
func action(n1 int, n2 int, operation func(int, int) int){
    result := operation(n1, n2)
    fmt.Println(result)
}
func main() {
     
    action(10, 25, func (x int, y int) int { return x + y })    // 35
    action(5, 6, func (x int, y int) int { return x * y })      // 30
}

//_ АНОНИМНАЯ ФУНКЦИЯ КАК РЕЗУЛЬТАТ
func selectFn(n int) (func(int, int) int){
    if n==1 {
        return func(x int, y int) int{ return x + y}
    }else if n==2{
        return func(x int, y int) int{ return x - y}
    }else{
        return func(x int, y int) int{ return x * y}
    }
}
 
func main() {
    f := selectFn(1)
    fmt.Println(f(2, 3))        // 5
    fmt.Println(f(4, 5))        // 9
    fmt.Println(f(7, 6))        // 13
}

//_ ДОСТУП К ОКРУЖЕНИЮ
// анонимные функции имеют доступ к окружению
// в коттором они определяются
// но значение которое замыкается в функции 
// оценивается каждый раз при ее использовании
// и если оно поменяется в какой то момент
// то и результат функции изменится
func square() func() int { 
    var x int = 2
    return func() int { 
        x++
        return x * x
    }
}
func main() {
     
    f := square()
    fmt.Println(f())        // 9
    fmt.Println(f())        // 16
    fmt.Println(f())        // 25
}