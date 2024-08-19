//_ БАЗОВЫЕ ОПЦИИ
//~ метод
// -X, --request - метод запроса, еали не указать GET   
curl -X POST localhost:3000 

//~ хедеры
// -Н, --header - доп заголовок, может быть сколько угодно   
curl -H "X-Name: Sam" -H "X-Age: 15" localhost:3000

//~ текстовые данные
// синоним --data - текстовые данные, упадут в body, 
// если написать переменные в стиле URL, 
// они попадут в Form и Postform
// encode будет равен application/x-www-form-urlencoded
// метод будет POST всегда
curl -X -d "name=Joe&age=15" localhost:3000 

//~ форма
// -F, --form - эмулирует форму, можно отправлять бинарные файлы
// encode всегда multipart/form-data
// метод всегда POST
curl -F name=Sara -F age=12 localhost:3000 

// отправка файла
// обязательно нужна собака в пути
curl -F "userid=1" -F "filecomment=This is an image file" -F "file=@/store/dev/knowlege/README.md" localhost:3000

//~ аутентификация
// -u <user:password>, --user <user:password> - пользователь и пароль для идентификации на сервере

//~ заголовки в дамп файл
// -D, --dump-header - кладет заголовки в указанный дамп файл 
curl -X GET "https://httpbin.org/get" -D dump.headers

//~ body в файл
// -o, --output - кидает полученный BODY в файл
curl -X GET "https://httpbin.org/get" -o dump.txt

//_ ПРАКТИКА GET запросы
curl -X GET "https://httpbin.org/get" \
-H "accept: application/json" \
-D result.headers \
-o result.json

//_ ПРАКТИКА POST запросы
//~ отправка простого текста
curl -X POST "https://httpbin.org/post" \
-H "accept: application/json" \
-H "Content-Type: text/plain" \
-H "Custom-Header: Testing" \
-d "I love hashnode" \
-D result.headers \
-o result.json  

//~ отправка файла json
curl -X POST "https://httpbin.org/post" \
-H "Content-Type: application/json; charset=utf-8" \
-d @data.json \
-o result.json

// файл data.json
{
    "name": "Jane",
    "last": "Doe"
}

//~ эмуляция отправки формы
curl -X POST "https://httpbin.org/post" \
-H "Content-Type: multipart/form-data" \
-F "FavoriteFood=Pizza" \
-F "FavoriteBeverage=Beer" \
-o result.json

//~ отправка файла
curl -X POST "https://httpbin.org/post" \
-H "Content-Type: multipart/form-data" \
-F "FileComment=This is a JPG file" \
-F "image=@image.jpg" \
-o result.json

//~ аутентификация 
curl -X GET "https://httpbin.org/basic-auth/carlos/secret" \
-u carlos:secret
-H "accept: application/json" \
-D result.headers