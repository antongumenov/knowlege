//_ 1 - УСТАНОВКА
paru -S tailwindcss-bin

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