//_ СУТЬ
// когда я хочу 
// дать горутине время на выполнение
// ограничить это время
// иметь возможность прервать ее извне
// или извне передать в горутину какие то данные(окуратно)
// использую контекст

//_ СТРУКТУРА context.Context
// имеет методы
// Value(key) - возвращает значение засунутое в контекст по ключу

// Done() - возвращает канал, по которому можно получить уведомление по отмене 
// внутри функции, в которую передан контекст
// я могу проверить, отменен ли контекст, и выбрать действие

// Deadline() - возвращает time.Time() когда крайний срок у запроса

// вернет либо Canseled(контекст был отменен), либо DeadlineExeeded(время контекста истекло)
// Err() - если я хочу узнать почему сработал Done, использую эту функцию

//_ МЕТОДЫ СОЗДАНИЯ КОНТЕКСТОВ ИСХОДЯ ИЗ ЗАДАЧ
// Background() - создает коренной контекст, его делать нужно всегда, 
// и все остальные контексты делаются из него

// WithCansel(ctx) - создает контекст которому можно сигналить отмену 
// WithDeadline(ctx, time) - создает контекст который прикращает работу в заданное time
// WithTimeout(ctx, duration) - чоздает контекст который отменится после истечения duration
// WithValue(ctx, key, val) - создает контекст с вложенной парой ключ значение

//! ПРИ ЭТОМ МОЖНО СОЗДАТЬ КОНТЕКСТЫ ОДИН ИЗ ДРУГИХ
//! НАПРИМЕР WithCansel ИЗ Background, ИЗ WithCansel СОЗДАТЬ WithValue, 
//! ПЕРЕДАВ ОДНО ЗНАЧЕНИЕ, ПОТОМ СНОВА ИЗ WithValue ЕЩЕ WithValue
//! ДОБАВИВ ЕЩЕ ОДНО ЗНАЧЕНИЕ 

//_ Context С ОТМЕНОЙ WithCansel(ctx)
// когда я просто хочу отменить горутину когда мне нужно
func processRequest(ctx context.Context, wg *sync.WaitGroup) {
	// когда мне необходимо
	// проверяю не отменен ли контекст
	select {
	// если отменен идем в конец и завершаем горутин
	case <-ctx.Done():
		goto end
	default:
		// иначе делаем какое то действие
	}
end:
	wg.Done()
}
func main() {
	waitGroup := sync.WaitGroup{}
	waitGroup.Add(1)
	// создаю контекст с отменой из контекста по умолчанию
	// получаю так же функцию отмены
	ctx, cancel := context.WithCancel(context.Background())
	// передаю в горутину
	go processRequest(ctx, &waitGroup)
	// когда нужно отменяю горутину
	cancel()
	waitGroup.Wait()
}


//_ Context c ТАЙМАУТОМ WithTimeout(ctx, duration) 
//_ Context с КРАЙНИМ СТРОКОМ WithDeadline(ctx, time)
//_ Canseled, DeadlineExeeded
// когда я хочу дать горутине определенный промежуток времени на работу
// или определить крайний срок работы горутины
// Canseled, DeadlineExeeded - определяют по какой причине был отменен контекст
func processRequest(ctx context.Context, wg *sync.WaitGroup) {
	// проверяю не отменен ли контекст
	select {
	case <-ctx.Done():
		if ctx.Err() == context.Canceled {
			// если отменен делаем что-то
		} else {
			// если истекло время делаем что-то другое
		}
		// и завершаем горутину
		goto end
	default:
	}

end:
	wg.Done()
}
func main() {
	waitGroup := sync.WaitGroup{}
	waitGroup.Add(1)
	// создаю контекст и передаю в горутину
	// и даю 2 секунды на работу горутине
	// получаю так же функцию отмены
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	go processRequest(ctx, &waitGroup)
	waitGroup.Wait()
}

// _ ВЛОЖЕНИЕ ДАННЫХ В Context WithValue(ctx, key, val)
// когда нужно вложить в контекст какое то значение
// чтобы преедать в какие то горутины или серию горутин
// как правило кроме какого нибудь id вкладываать что-то плохая практика
func processRequest(ctx context.Context, wg *sync.WaitGroup) {
	// получаю первое значение
	count := ctx.Value("id").(int)
	sleepPeriod := ctx.Value("time").(time.Duration)
	select {
	// если контекст отменен можно попытаться понять причину и завершить горутину
	case <-ctx.Done():
		if ctx.Err() == context.Canceled {
		} else {
		}
		goto end
	// иначе сделать что-то
	default:
	}
end:
	wg.Done()
}
func main() {
	waitGroup := sync.WaitGroup{}
	waitGroup.Add(1)
	// создаю контекст с таймаутом
	// получаю так же функцию отмены
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*2)
	// в него вкладываю первое значение
	ctx = context.WithValue(ctx, "id", 4)
	// второе значение
	ctx = context.WithValue(ctx, "time", time.Millisecond*250)
	// отдаю его в горутину
	go processRequest(ctx, &waitGroup)
	waitGroup.Wait()
}
