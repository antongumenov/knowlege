//>> ДАННЫЕ ПОЛЕТЯТ В КУЧУ В СЛЕДУЮЩИХ СИТУАЦИЯХ
//<< возврат и передача знаяений по ссылке
// когда я делюсь знаячением между стеками 
// возвращаю значение из функции по ссылке 
// или передаю его в функцию по ссылке
// go кидает его в кучу

// проверить куда go кладет значения 
go build -gcflags "-m -m"

func retNum() *int {
	i := 1
	return &i//здесь я делюсь значением и оно улетает в heap
}

func retNum2(i *int) { //здесь я делюсь значением и оно улетает в heap
	fmt.Println(i)
}

func main() {
	fmt.Println(retNum())//здесь оно улетает в heap
    i := 5
	retNum(&i)//здесь оно улетает в heap
}

//<< возврат и передача значений как интерфесов
// если функция получает или передает значение как тип интерфейса
// оно улетит в кучу

// по этому
Используйте интерфейс, когда:
    - пользователи API должны предоставить детали реализации.
    - у API есть несколько реализаций, которые они должны поддерживать внутри.
    - были определены части API, которые могут измениться и требуют разделения.
Не используйте интерфейс:
    - ради использования интерфейса.
    - для обобщения алгоритма.
    - когда пользователи могут объявить свои собственные интерфейсы.

//<< определение срезов с неопределенным размером
size := len(find) // здесь размер определяется в run time а не в compile time
buf := make([]byte, size) |// по этому buf улетит в кучу

buf := make([]byte, 5)// а так в стек

//! Вы не должны писать программу с нулевым количеством аллокаций, 
//! но должны стремиться минимизировать аллокации, когда это возможно. 
//! Это означает, что в первую очередь нужно сосредоточиться 
//! на целостности, удобочитаемости и простоте. 
//! После того, как у вас есть рабочая программа, 
//! определите, достаточно ли она быстра. 
//!Если нет, используйте инструменты, предоставляемые языком, 
//! чтобы найти и исправить проблемы с производительностью.

//>> ПРАВИЛА
1. Примитивные типы передавай и принимай по значению, за редким исключением.
2. Ссылочные типы slice, map, interface, function и channel передавай и принимай по значению, за редким исключением.
3. Пользовательские типы ...ЧИТАЙ https://www.ardanlabs.com/blog/2017/06/design-philosophy-on-data-and-semantics.html