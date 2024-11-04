//_ ХРАНЕНИЕ ОТДЕЛЬНОГО СОСТОЯНИЯ
// когда состояние приложения необходимо для нескольких компонентов
// оно хранится в отдельном модуле
// в свелт есть втроенный функционал для стейта приложения
// я могу подписаться
// и компонент будет обновляться

//_ WRITABLE
// состояние которое я могу изменять

//~ создание
// создаю файл stores.js
import { writable } from "svelte/store";

// count - это обьект который содержит в себе скрытый начальный стейт
// и методы подписки, обновления и установки значения
export const count = writable(0);

//~ подписка и чтение
// в компоненте
<script>
	import { onDestroy } from 'svelte';
	import { count } from './stores.js';
	import Incrementer from './Incrementer.svelte';
	import Decrementer from './Decrementer.svelte';
	import Resetter from './Resetter.svelte';

	let count_value;

    // подписываюсь, одновременно получаю колбек чтоб отписаться
	const unsubscribe = count.subscribe(value => {
		count_value = value;
	});

    // отписываюсь
	onDestroy(unsubscribe);
</script>

<h1>The count is {count_value}</h1>

<Incrementer />
<Decrementer />
<Resetter />

//~ изменение
// в компоненте
<script>
	import { count } from './stores.js';

	function decrement() {
		count.update((n) => n - 1);
	}
</script>

<button on:click={decrement}>
	-
</button>

//~ установка значения
// в компоненте
<script>
	import { count } from './stores.js';

	function reset() {
		count.set(0);
	}
</script>

<button on:click={reset}>
	reset
</button>

//_ АВТОПОДПИСКА
// это синтаксический сахар генерирующий и подписку и отписку
// в компоненте
<script>
	import { count } from './stores.js';
	import Incrementer from './Incrementer.svelte';
	import Decrementer from './Decrementer.svelte';
	import Resetter from './Resetter.svelte';
</script>

// на этой строке сгенерируется и подписка и отписка
<h1>The count is {$count}</h1>

<Incrementer />
<Decrementer />
<Resetter />

//_ READABLE
// состояние которое не нужно изменять в компонегтах
// например положение мышки или время

//~ создание 
// в stores.js
import { readable } from "svelte/store";

// первый аргумент в readable это начальное значение
// второй аргумент функция start, которая запускается при первой подписке
// принимающая колбек set для установки значения
// и возвращающая функцию stop, которая запускается когда последний отписался
export const time = readable(new Date(), function start(set) {
	const interval = setInterval(() => {
		set(new Date());
	}, 1000);

	return function stop() {
		clearInterval(interval);
	};
});

//~ автоподписка
// в компоненте
<script>
	import { time } from './stores.js';

	const formatter = new Intl.DateTimeFormat(
		'en',
		{
			hour12: true,
			hour: 'numeric',
			minute: '2-digit',
			second: '2-digit'
		}
	);
</script>

<h1>The time is {formatter.format($time)}</h1>


//_ ВЫВОДИМОЕ ИЛИ ЗАВИСИМОЕ СОСТОЯНИЕ
// можно задать состояние которое зависит от другого состояния

//~ например создание readable + derived
// в stores.js
import { readable, derived } from "svelte/store";

export const time = readable(new Date(), function start(set) {
	const interval = setInterval(() => {
		set(new Date());
	}, 1000);

	return function stop() {
		clearInterval(interval);
	};
});

const start = new Date();
// запоминает начальное время
// подписывается на readable state 
// и расчитывает свое значение
export const elapsed = derived(
	time,
	($time) => Math.round(($time - start) / 1000)
);

//~ использование
// в компоненте
<script>
	import { time, elapsed } from './stores.js';

	const formatter = new Intl.DateTimeFormat(
		'en',
		{
			hour12: true,
			hour: 'numeric',
			minute: '2-digit',
			second: '2-digit'
		}
	);
</script>

<h1>The time is {formatter.format($time)}</h1>

<p>
	This page has been open for
	{$elapsed}
	{$elapsed === 1 ? 'second' : 'seconds'}
</p>

//_ КАСТОМНЫЙ СТЕЙТ
//~ создание
// я могу создать и имплементировать логику в нем же
// ту которую я захочу
// в stores.js
import { writable } from "svelte/store";

function createCount() {
	const { subscribe, set, update } = writable(0);

	return {
		subscribe,
		increment: () => {},
		decrement: () => {},
		reset: () => {}
	};
}

// создаю обьект типа count
export const count = createCount();

//~ простое применения
<script>
	import { count } from './stores.js';
</script>

<h1>The count is {$count}</h1>

<button on:click={count.increment}>+</button>
<button on:click={count.decrement}>-</button>
<button on:click={count.reset}>reset</button>


//_ БАЙНДИНГ
// состояние можно так же привязывать
// например создаю такое сотсояние
import { writable, derived } from "svelte/store";

export const name = writable('world');

export const greeting = derived(name, ($name) => `Hello ${$name}!`);

// привязываю через присваивание с автоподпиской
<script>
	import { name, greeting } from './stores.js';
</script>

<h1>{$greeting}</h1>
// привязываю в input с автоподпиской
<input bind:value={$name} />

<button on:click={() => $name += '!'}>
	Add exclamation mark!
</button>   