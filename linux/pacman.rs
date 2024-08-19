//_ УДАЛЕНИЕ ПАКЕТОВ
//~ удаление пакета со всеми зависимостями
sudo pacman -Rs name

//~ удаление ненужных пакетов и их конфигов
sudo pacman -Qtdq | pacman -Rns -

// более тщательно, здесь будут пакеты опционально используемые
pacman -Qqd | pacman -Rsu --print -
// удалить
pacman -Qqd | pacman -Rsu 
