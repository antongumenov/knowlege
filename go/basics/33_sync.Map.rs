//>> СУТЬ ПРОБЛЕМЫ
// когда большое количество горутин читает из расшариваемой мапы
// с помощью RWMutex
// скорость мапы падает
// типа ести это 100 или 200 горутин
// в таких случаях связку мапа плюс RWMutex не стоит использовать
// а юзать sync.Map
// это та же мапа только для конкурентного чтения и записи не нужно использовать RWMutex
// но реальный выигрышь только на машинах с более чем 5 ядрами

//! НО ЕСТЬ И ДРУГИЕ ПАКЕТЫ НА LOCK_FREE АЛГАРИТМАХ

// обьявление
var m sync.Map

// запись
m.Store("habr", 16)

// чтение 
v, ok = m.Load("habr")

// удаление
m.delete("habr")

// получить значение и если его нет сохранить
v, ok = m.LoadOrStore("habr")

// перебор
m.Range(func (k, v interface{}) bool{
    fmt.Println("key", k, "val", v)
    return true // если false то перебор остановится
})  