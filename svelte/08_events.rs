//_ СОБЫТИЯ
<script>
	let m = { x: 0, y: 0 };

	function handleMove(event) {
		m.x = event.clientX;
		m.y = event.clientY;
	}
</script>

// задаю реакцию
<div on:pointermove={handleMove}>
	The pointer is at {m.x} x {m.y}
</div>

//_ МОДИФИКАТОРЫ СОБЫТИЙ
// можно задать характеристики реакции на событие
// тут она сработает лишь раз
<button on:click|once={() => alert('clicked')}>
	Click me
</button>

//~ список модификаторов 
// preventDefault — event.preventDefault() 
// stopPropagation — event.stopPropagation()
// passive — улутшает производительность прокрутки, свелт добавляет сам где может
// nonpassive — passive: false
// capture — сработает при возникновении события но не при всплытии
// once — удаляет хендлер после первого события
// self — сработает тольео если event.target является самим элементом
// trusted — сработает только если  event.isTrusted = true, то есть только при  действии пользователя

// можно группировать
on:click|once|capture={...}

//_ СОБЫТИЯ КОМПОНЕНТА
// можно создавать собственные события

//~ компонент в котором создаю событие
<script>
	import { createEventDispatcher } from 'svelte';

    // создаю диспетчер
	const dispatch = createEventDispatcher();

    // при вызове этой функции запускаю событие
	function sayHello() {
		dispatch("message", {
			text: "Hello!"
		});
	}
</script>

<button on:click={sayHello}>
	Click to say hello
</button>


//~ компонент в котором ловлю событие
<script>
	import Inner from './Inner.svelte';

	function handleMessage(event) {
		alert(event.detail.text);
	}
</script>

// ловлю событие
<Inner on:message={handleMessage} />

//_ ВСПЛЫТИЕ СОБЫТИЙ КОМПОНЕНТА
// в отличии от DOM событий, события компонента не всплывают
// его нужно пробрасывать если хочу поймать его выше

//~ компонент в котором создаю событие
<script>
	import { createEventDispatcher } from 'svelte';

    // создаю диспетчер
	const dispatch = createEventDispatcher();

    // при вызове этой функции запускаю событие
	function sayHello() {
		dispatch("message", {
			text: "Hello!"
		});
	}
</script>

<button on:click={sayHello}>
	Click to say hello
</button>

//~ здесь использую компонент с событием
<script>
	import Inner from './Inner.svelte';
</script>

// пробрасываю
<Inner on:message /> 

//~ использую
<script>
	import Outer from './Outer.svelte';

	function handleMessage(event) {
		alert(event.detail.text);
	}
</script>

<Outer on:message={handleMessage} />

//_ ПРОБРАСЫВАНЕИ DOM СОБЫТИЙ
// здесь использую внешний обработчик
<button on:click>
	Push
</button>

// пробрасываю обработчик в кнопку
<script>
	import BigRedButton from './BigRedButton.svelte';

    function handleClick() {
		console.log("fire")
	}
</script>

<BigRedButton on:click={handleClick} />