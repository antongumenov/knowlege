//>> ПРАВИЛА НОРМАЛИЗАЦИИ
//<< 1  нормальная форма
// один столбец может содержать только одно значение

//<< 2 нормальная форма
// применяется только с таблицами с составными ключами
// атрибуты должны зависеть от первичного ключа полностью
// если ключь из нескольких значений, а атрибут зависит только от одного, 
// такие значения нужно выносить в отдельную таблицу

//<< 3 нормальная форма
// в таблице не должно быть чтобы один неключевой столбец зависел от другого не ключевого 
// менеджер и его телефон не должны быть в одной таблице, 
// в которой оба не являются первичными ключами