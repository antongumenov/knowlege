//_ РАБОТА С ОТВЕТАМИ В ХЕНДЛЕРЕ

//~ Установка Хедеров
// метод возвращает Header который по сути map[string][]string
// для того чтобы установить хедеры
header := writer.Header()
header[`my_header`] = []string{`my_header_string`}

//~ Установка статуса ответа
// статусы ответа можно писать константами
writer.WriteHeader(http.StatusOK)
// или так
writer.WriteHeader(200)

//~ Запись в BODY
writer.Write([]byte(sh.message))

//_ УДОБНЫЕ ФУНКЦИИ ОТВЕТА
// их нужно использовать в теле Хандлера

//~ Возврать текстовой ошибки
// возвразщает текстовую ошибку, которая отбразиться в браузере как текст
http.Error(writer, "Че то не так", 500)

//~ Возврат ошибки NotFound
// возвразщает текстовую ошибку, которая отбразиться в браузере как 404 page not found
http.NotFound(writer, request)

//~ Редирект
// редирект на path с кодом ошибки
// если редирект с /, выведет код ошибки как ссылку на /auth
http.Redirect(writer, request, `auth`, 500)

//~ Отправка файла
// отправит файл и отобразит в браузере
http.ServeFile(writer, request, `1.jpg`)

// можно отправить html
http.ServeFile(writer, request, `index.html`)

//_ ОТПРАВКА JSON В ОТВЕТ
// чтобы отправить ответ как json нужно задать соответствующий хедер
writer.Header().Set("Content-Type", "application/json")
// и создаеть Encoder и закодировать данные 
// и они запишуться в ответ
json.NewEncoder(writer).Encode(Products)
