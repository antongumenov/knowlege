//_ КОНТЕКСТ
// для общения разных компонентов напрямую
// без цепочек пробросов

//~ установка значения в одном компоненте
import { setContext } from 'svelte';

// просто добавляю значение по ключу
setContext('name', 'john');


//~ получение значения в другом и проверка на наличие
import { getContext, hasContext } from 'svelte';

let name;

if (hasContext('name')){
    name=getContext('name');
}

//_ РЕАКТИВНОСТЬ
// по умолчанию переменные контекста не реактивны
// если хочешь сделать реактивным, передавай store