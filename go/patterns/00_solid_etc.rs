//_ ПРИНЦИПЫ ПОСТРОЕНИЯ ПРОГРАМНОГО ОБЕСПЕЧЕНИЯ

//~ SOLID

// S -  Принцип единственной ответственности 
//      - каждая функция и фича 
//      должна заниматься только одной задачей 
//      и иметь только одну причину для изменения.
//      Проектируя по нужно дробить все по задачам.

// O -  Принцип открытости/закрытости 
//      - програмные объекты должны быть открыты для расширения 
//      и закрыты для модификации. 
//      Проектируй так, чтобы код не нужно было менять.
//      ПРОЕКТИРУЙ ФУНКЦИОНАЛ ПРИНИМАЮЩИЙ ИНТЕРФЕЙСЫ, И ЗАПУСКАЮЩИЙ МЕТОДЫ ИНТЕРФЕЙСОВ
//      тогда для расширения нужно будет просто создать свой тип и реализовать метод интерфейса
//      и исходный код будет не тронутым, но он будет открыт к расширению
//      через создание интерфейсов

import "fmt"
type PaymentMethod interface {
  Pay()
}
// структура платежа запускает метод передаваемого типа, а какой он будет, пофиг
// главное чтобы реализовывал интерфейс
type Payment struct{}
func (p Payment) Process(pm PaymentMethod) {
  pm.Pay()
}
type CreditCard struct {
  amount float64
}
func (cc CreditCard) Pay() {
  fmt.Printf("Paid %.2f using CreditCard", cc.amount)
}
func main() {
  p := Payment{}
  cc := CreditCard{12.23}
  p.Process(cc)
}

// L -  Принцип подстановки Барбары Лисков
//      - два типа являются взаимозаменяемыми 
//      если они проявляют такое поведение 
//      при котором вызывающий не может определить разницу.
//      В го это реализуется с помощью интерфейсов.
//      Вместо типов, передавай интерфейсы, если входящие типы могут быть заменены другими со временем

// I -  Принцип разделения интерфейса 
//      - принимаемый интерфейс должен описывать только тот функционал
//      который используется внутри функции
//      НЕ ДЕЛАЙ ОГРОМНЫХ ИНТЕРФЕЙСОВ
//      они заставляют реализовывать весь функционал в каждой реализующей интерфейс структуре
//      а если что то их этого функционала не нужно, зачем городить этот код
//      и если добавить новый метод в интерфейс, опять все менят

// D -  Принцип инверсии зависимостей 
//      - пакеты верхних уровней не должны зависить 
//      от пакетов нижних уровней
//      если в структуре(например в контроллере) 
//      используется другая структура(например репозиторий)
//      которая может быть чем то заменена со временем
//      НЕ ВСТРАИВАЙ ДОЧЕРНЮЮ СТРУКТУРУ ПО ТИПУ, 
//      ЕСЛИ НЕ ХОЧЕШЬ РАСШИРЯТЬ ЕЕ ФУНКЦИОНАЛ
//      ВСТРАИВАЙ ЧЕРЕЗ ИНТЕРФЕЙС

//~ DRY
// в коде не должно быть повторяемой логики
// которая если меняется, требует изменения в двух местах 