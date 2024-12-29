//_ СЛАЙС
// это структура, которая содержит
// указатель на первый элемент базового массива
// длинну среза - сколько уже лежит
// емкость среза - сколько можно положить
//! слайсы предаються в функию по cсылке

//~ обьявление
// здесь под капотом будет создан массив из 3 элементов
// но с емкостью 10 элементов
// по умолчанию будет три нулевых строки
// и в names упадет ссылка на этот массив
names := make([]string, 3, 10)

// вставляем значения
names[0] = "Kayak"
names[1] = "Lifejacket"
names[2] = "Paddle"

//~ создание среза с помощью литерального синтаксиса
// здесь так же будет создан базовый массив с 3 элементами0
// и в names упадетссылка на него 
names := []string {"Kayak", "Lifejacket", "Paddle"}

//_ СОЗДАНИЕ СРЕЗА ИЗ СРЕЗА ИЛИ МАССИВА
// позволяет создать срез из массива
initialUsers := [8]string{"Bob", "Alice", "Kate", "Sam", "Tom", "Paul", "Mike", "Robert"}
users1 := initialUsers[2:6]           // с 3-го по 6-й
fmt.Println(len(users1), cap(users1)) // 4 6 (емкость 8 базового -2)

users2 := initialUsers[:4]            // с 1-го по 4-й
fmt.Println(len(users2), cap(users2)) // 4 8 (емкость 8 базового -0)

users3 := initialUsers[3:]            // с 4-го до конца
fmt.Println(len(users3), cap(users3)) // 5 5 (емкость 8 базового -3)

users4 := initialUsers[:]             // все
fmt.Println(len(users4), cap(users4)) // 8 8 (емкость все 8 базового)

//~ с указанием емкости
// последняя цифра говорит сколько значений взять с начала базового массива
users5 := initialUsers[2:5:5]
fmt.Println(len(users5), cap(users5)) // 3 3 (емкость 5 базового -2)

users6 := initialUsers[2:5:6]
fmt.Println(len(users6), cap(users6)) // 3 4 (емкость 6 базового -2)

//_ ВСТАВКА В СРЕЗ
//~ вставка элементов
// вставка в конец 
// вставляем два элемента 
names = append(names, "Hat", "Gloves")  

//~ вставка среза
arr := []string{"man", "woman", "child"}
arr2 := []string{"granny", "friend", "girl"}
arr = append(arr, arr2...)

//~ вставка части среза
arr = append(arr, arr2[:2]...) // вставить все элементы кроме последнего

//_ УДАЛЕНИЕ ЭЛЕМЕНТА
users := []string{"Bob", "Alice", "Kate", "Sam", "Tom", "Paul", "Mike", "Robert"}
//удаляем 4-й элемент
var n = 3
users = append(users[:n], users[n+1:]...)   
fmt.Println(users)      //["Bob", "Alice", "Kate", "Tom", "Paul", "Mike", "Robert"]

//_ НЮАНСЫ ДЛИННЫ И ЕМКОСТИ
//~ если срез полностью заполнен
arr := []string{"man", "woman", "child"}
fmt.Println(len(arr), cap(arr))// 3 3 
// при добавлении нового элемента
// его некуда вставлять
// по этому под капотом аллоцируется новый базовый массив
// емкостью в два раза больше(до 1024 байт, затем +12,5%)
// туда копируется старый массив и новые элементы
arr = append(arr, "granny")
fmt.Println(len(arr), cap(arr))// 4 6
// и это очень дорогая операция
// старый массив будет удален GC и это тоже нагрузка

//! ЕСЛИ БУДЕШЬ ВСТАВЛЯТЬ ЭЛЕМЕНТЫ В СРЕЗ
//! СРАЗУ УКАЖИ ПРЕДПОЛАГАЕМУЮ ЕМКОСТЬ
//! ЧТОБЫ ИСКЛЮЧИТЬ ПОСТОЯННЫЕ АЛЛОКАЦИИ

//~ если срез на массив большей длинны
// если я взял массив путь 5 элементов
// взял в слайс первые пусть два
// то каждое добавление элементов в слайс 
// будет перезаписывать следующий элемент базового массива
// и если на него ссылается другой слайс
// он так же будет изменен 

//_ КОПИРОВАНИЕ СРЕЗОВ
// необходимо для создания независимых срезов

products := [4]string{"Kayak", "Lifejacket", "Paddle", "Hat"}
allNames := products[1:]
someNames := make([]string, 2)
// в someNames скопировать allNames сколько влезет
copy(someNames, allNames)            
fmt.Println("someNames:", someNames) // someNames: [Lifejacket Paddle]
fmt.Println("allNames", allNames)    // allNames [Lifejacket Paddle Hat]

// если поменять элемент одного среза, второй срез не изменится
allNames[1] = "Jacket"
fmt.Println("someNames:", someNames) // someNames: [Lifejacket Paddle]
fmt.Println("allNames", allNames)    // [Lifejacket Jacket Hat]

// при копировании можно указывать и диапазоны
copy(someNames[1:], allNames[2:3])

//_ ИТЕРАЦИЯ ПО СРЕЗАМ
// так же как и по массивам
for index, value := range products[2:] {
    fmt.Println("Index:", index, "Value:", value)
}
 
//_ СРАВНЕНИЕ СРЕЗОВ
// только с помощью покета reflect
p1 := []string { "Kayak", "Lifejacket", "Paddle", "Hat"}
p2 := p1
fmt.Println("Equal:", reflect.DeepEqual(p1, p2))// Equal: true

//_ ПОЛУЧЕНИЕ БАЗОВОГО МАССИВА
p1 := []string { "Kayak", "Lifejacket", "Paddle", "Hat"}
arrayPtr := (*[3]string)(p1) // явное приобразование слайса в указатель
array := *arrayPtr// получение значения по указателю
fmt.Println(array) // [Kayak Lifejacket Paddle]

//_ ИЗМЕНЕНИЕ ДЛИННЫ СРЕЗА
// Изменение длины среза
slice2 = slice2[:2]
fmt.Println("Срез 2 (измененная длина):", slice2)

//_ ПОЛЕЗНЫЕ ПРАКТИКИ
//~ проверяй слайс на пустоту так len(list)==0

//~ не создавай пустые слайсы, 
// если планируешь добавлять элементы 
// всегда задавай емкость
// чтобы исключить доп аллокаций при добавлении каждого элемента

//~ не используй append для создания нового массива
// никогда не пиши arr2 := append(arr, val)
// только так arr = append(arr, val)
// так как append вернет ссылку на старый базовый массив 
// если емкость не привышена, и ты не получишь новый 
// и ссылку на новый базовый если привышена
// из-за этого могут быть проблемы и путоница

//~ не возвращай слайсы на массивы большой длинны, возвращай копию
// например, 
// пусть ты искал в функции 
// по регулярному выражению в строке из файла весом 10гб
// результатом может быть строка 10 символов или слайс 10 байт
// если ты вернешь этот слайс, 
// базовым слайсом так и будет слайс 10гб
// GC не будет освобождать память
// и пока существует этот слайс память будет занята
// и если ты хочешь поискать еще несколько раз, рискуешь сожрать памчять
// по этому всегда создавай новый слайс, клади туда результат и возвращай его
// а GC отчистит 10гб после выхода их функции 

//_ НЮАНСЫ ПЕРЕДАЧИ СЛАЙСОВ В ФУНКЦИЮ
//~ передача по значению
// копируется структура среза (в которой указатель на срез)
// сама структура не копируется
// если изменить емкость или длинну эти изменения не изменят коренную структуру среза

//~ при передаче по ссылке 
// если изменить емкость или длинну эти изменения изменят коренную структуру среза
