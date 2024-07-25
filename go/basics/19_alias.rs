//_ ПРОИЗВОДНЫЕ ТИПЫ И ПСЕВДАНИМЫ
//~ производные типы
// тип на основе другого типа
// сним можно делатиь все что с uint
// но вместо него использовать uint нельзя
type mile uint

//~ псевданим
// в отличии от производного типа не является отдельным типом
type logLevel = uint

func printLogLevel(ll logLevel) {
	fmt.Println(ll)
}
func main() {
	var ui uint = 5
    // я должен передать logLevel
    // но могу передать и uint
	printLogLevel(ui)
}