//_ СИСТЕМА
// src/routes - тут файлы начального экрана
// src/routes/about - файлы страницы about
// src/routes/blog/[slug] - файлы динамического пути типа /blog/hello-world

//_ ФАЙЛЫ В ПАПКЕ МАРШРУТА
//~ +page.svelte
// файл компонента
// по умолчанию рендерятся как на клиенте так и на сарвере

//~ +page.js
// здесь происходит загрузка данных перед отрисовкой
// с помощью функции load

//~ +page.server.js
// все что сдесь отработает только на сервере
// например сходить в базу данных
// с помощью функции load

//~ +error.svelte
// если во время load произошла ошибка
// отрендерится эта страница

//~ +layout.svelte
// здесь компонент лейаута
// если положить в src/routes
// то он будет работать для всех страниц

<nav>
	<a href="/">Home</a>
	<a href="/about">About</a>
	<a href="/settings">Settings</a>
</nav>

<slot></slot>

// в slot попадут все компоненты страниц

//~ +layout.js
// то же самое что и +page.js но для лейаута

//~ +layout.server.js
// то же самое что и +page.server.js но для лейаута

//~ +server.js
// это логика сервера
// создает эндпоинт по этому пути
