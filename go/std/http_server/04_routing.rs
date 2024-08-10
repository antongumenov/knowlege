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