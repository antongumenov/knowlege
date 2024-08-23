//_ ОДНОКРАТНОЕ ВЫПОЛНЕНИЕ ФУНКЦИИ
// когда у меня есть данные которые нужно подготовить один раз
// и когда есть функции которые используют эти данные
// и соответственно должны подождать пока они не приготовятся
// и когда подготовка данных не обязательно должна быть в отдельной горутине
// я могу подготовить их в первой горутине, вызвав функцию которая их готовит
// но так как я не знаю какая горутина запустится первая
// мне прийдется в каждой горутине запускать приготовление данных
// и чтобы не городить проверки
// можно просто использовать sync.Once
// которая имеет метод Do(func), который принимает функцию
// и сразу запускает ее только один раз

// создаю группу ожидания
var waitGroup = sync.WaitGroup{}

// создаю sync.Once
var once = sync.Once{}

// и расшариваемые данные
var squares = map[int]int{}

// функция подготовки значений
func generateSquares(max int) {
	fmt.Println("Generating data...")
	for val := 0; val < max; val++ {
		squares[val] = int(math.Pow(float64(val), 2))
	}
}

// функция использующая значения
func readSquares(id, max, iterations int) {
	// сколько бы раз не запустилась бы эта функция
	// генерировать значения будем один раз
	once.Do(func() {
		generateSquares(max)
	})
	for i := 0; i < iterations; i++ {
		key := rand.Intn(max)
		fmt.Printf("#%v Read value: %v = %v\n", id, key,
			squares[key])
		time.Sleep(time.Millisecond * 100)
	}
	waitGroup.Done()
}
func main() {
	numRoutines := 2
	waitGroup.Add(numRoutines)
	for i := 0; i < numRoutines; i++ {
		go readSquares(i, 10, 5)
	}
	waitGroup.Wait()
	fmt.Printf("Cached values: %v\n", len(squares))
}