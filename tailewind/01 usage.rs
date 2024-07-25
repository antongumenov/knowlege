//_ 1 - УСТАНОВКА
curl -sLO https://github.com/tailwindlabs/tailwindcss-linux-x64
chmod 755 tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss

// менняю пользователя на root
mv tailewindcss /usr/bin

// идем в .bahrc
// добавляем переменную среды в конец файла bash
PATH=$PATH:/usr/bit/tailwindcss
export PATH

//_ 2 - ГЕНЕРАЦИЯ tailwind.config.js
// tailwindcss init

//_ 3 - ПОПИСЫВАЮ ЛОКАЦИЮ ШАБЛОНОВ
// иду в tailwind.config.js
content: [
    './www/templates/*.html',
],

//_ 4 - ОФОРМЛЯЮ input.css
// в папке ststic/css создаю файл input.css
// вставляю в него базовые класы
@tailwind base;
@tailwind components;
@tailwind utilities;

//_ 5 - ИМПОРТНУТЬ СТИЛИ 
<link rel="stylesheet" href="../static/css/output.css">


//_ 6 - ЗАПУСК 
// watcher
tailwindcss -i static/css/input.css -o static/css/output.css --watch
// сборка
tailwindcss -i static/css/input.css -o static/css/output.css --minify