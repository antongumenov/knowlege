//_ МАРШРУТИЗАЦИЯ НА ОСНОВЕ Handler
//~ Handle(path, handler)
// добавляет в маршрутизатор обработчик пути
// на основе всего что реализует Handler
// то есть хендлер должен реализовивать матод ServeHTTP(writer, *request)
// принимает путь для обработки и все что реализует Handler
http.Handle("/", StringHandler{"Hello, World"})

//! при этом сервер запускаем с nil значением в Handler
http.ListenAndServe(":5000", nil)

//_ ХЕЛПЕРЫ НА ОСНОВЕ Handler
//~ NotFoundHandler
// вернет 404
http.Handle("/money", http.NotFoundHandler())

//~ RedirectHandler
// вернет страницу с ошибкой по статусу со ссылкой на /auth
http.Handle("/redir", http.RedirectHandler("auth", 500))

//~ StripPrefix
// вырезает префикс тут /strip, чтобы обработчик получил обрезанный URL
// как применять смотри static_server.rs
http.Handle("/strip/redir", http.StripPrefix(`/strip`, StringHandler{"strip"}))

//~ TimeoutHandler
// передаст запрос Handler но если ответа не будет в течении Duration вернет ошибку
http.Handle("/to", http.TimeoutHandler(StringHandler{"tomeout"}, 1*time.Second, "timeout"))

//_ МАРШРУТИЗАЦИЯ НА ОСНОВЕ ФУНКЦИИ
//~ HandleFunc(path, handlerFunc)
// позволяет передавать функцию как обработчик роута
// так же позволяет передать метод структуры, чтобы иметь состояние
// это дает возможность обьявлять сколько угодно хендлеров в одной структуре 
// Handler например StringHandler

// создаю экземпляр структуры в котором обработчики путей
sh := StringHandler{"My Message"}
// назначаю конкретный обработчик структуры на путь
http.HandleFunc("/", sh.someHandlerFunc)
// запускаю сервер
http.ListenAndServe(":5000", nil)

//_ SERVEMUX
// когда я использую методы http.Handle() или http.HandleFunc()
// под капотом создается глобальная переменная DefaultServeMux
// к ней и привязываются пути и обработчики
// это типа экономно, но не безопасно, 
// так как к глобальной переменной имеют доступ все пакеты
// по этому хорошая практика создавать свой, локальный ServeMux 
mux := http.NewServeMux()
// и уже к немк привязывать пути и обработчики
mux.HandleFunc("/", sh.someHandlerFunc)
// запускаю сервер
http.ListenAndServe(":5000", mux)

//_ ФИКСИРОВАННЫЕ И МНОГОУРОВНЕВЫЕ ПУТИ
//~ фиксированные
// фиксированнык типа /client/create - хэндлер сработает по точному совпадению

//~ многоуровневые
// многоуровневые типа /client/ - хендлер сработает с любым путем начинающимся на /client/
// в конце может быть вспомогательный символ, типа /client/*

//_ ОСОБЕННОСТИ SERVEMUX
//~ в приоритете более длинные пути
// пути можно регистрировать в любом порядке
// servemux отдает предпочтение более длинным
// например 
// если есть путь /product
// и /product/toy
// по запросу на .../product/toy сработает именно обработсик /product/toy

//~ все пути отчищаются
// при переходе на /foo/bar/..//baz
// будет сделан редирект на /foo/baz

//~ запрос на статический путь пойдет на многоуровневый если есть совпадения
// если определен /foo/ 
// любой запрос на /foo
// будет отправлен на /foo/