// _ ИСПОЛЬЗОВАНИЕ БЛОКИРОВОК ПО УСЛОВИЮ
// когда есть несколько горутин
// которые не только используют общие данные
// но одна горутина готовит данные и должна полюбому быть отдельной горутиной
// а остальные читают и работают уже с готовыми данными
// и соответственно ведущая горутина должна их подготовить
// а остальные подождать пока основная не даст сигнал
// используется блокировка по условию

// _ КАК ЭТО РЕАЛИЗОВАТЬ
// создаю мьютекс
var rwmutex = sync.RWMutex{}

//<< NewCond(*RWMutex)
// Cond всегда работает с в паре с RWMutex
var condLocker = sync.NewCond(rwmutex.RLocker())
// можно с простым мьютексом
//cond := sync.NewCond(&sync.Mutex{})

// создаю общие данные
var squares = map[int]int{}

// в основной горутине
func generateSquares(max int) {
	// блокирую
	rwmutex.Lock()

	// что то делаю

	// разблокирую
	rwmutex.Unlock()

	// даю всем кто ждет чтобы разблокировались и читали данные
	condLocker.Broadcast()
	// можно дать сигнал одной первой ждущей на очереди и сделать это несколько раз
	//condLocker.Signal()
}

// в зависимой горутине
func readSquares(id, max, iterations int) {
	// бокирую
	condLocker.L.Lock()

	// говорю ждем пока не будет дан сигнал
	// тут горутина разблокирует данные и сама блокируется, заняв очередь на блокировку
	condLocker.Wait()
	// если получил сигнал, читаю данные
	// здесь горутина получает блокировку над данными, если она первая

	// или ждет пока ее не отпустят предидущие
	// когда все сделал
	// разблокирую мьютекс и говорю что эта горутина отработала
	condLocker.L.Unlock()
}
func main() {
	// две зависимые горутины встанут пока не будет заполнены данные
	for i := 0; i < numRoutines; i++ {
		go readSquares(i, 10, 5)
	}
	// заполню данные и пробуждаю горутины
	go generateSquares(10)
	
	// тут конечно нужно подождать любыми возможными способами
}
