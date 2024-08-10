//_ ОБРАБОТЧИК СТАТИЧЕСКИХ ФАЙЛОВ
// создаем хендлер с каталогом откуда берем файлы
fsHandler := http.FileServer(http.Dir("./static"))
// обрежем путь, убрав /files, чтобы внутри FileServer
// сразу получать доступ к файлам
// по url - http://localhost:5000/files/index.html
// достанет файл по пути static/index.html
http.Handle("/files/", http.StripPrefix("/files", fsHandler))
http.ListenAndServe(":5000", nil)