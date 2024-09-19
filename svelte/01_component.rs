//_ КОМПОНЕНТ
// состояние компонента и логика живет здесь
<script>
	let name = 'Svelte';
</script>

// так состояние подключается в html динамически
// и автоматически поривязано к состоянию
// в скобках можно писать код
<h1>Hello {name.toUpperCase()}!</h1>

// здесь пишу стили
<style>
	p {
		color: goldenrod;
		font-family: 'Comic Sans MS', cursive;
		font-size: 2em;
	}
</style>

//_ ВСТРАИВАНИЕ КОМПОНЕНТОВ ИЗ ДРУГИХ ФАЙЛОВ

<script>
	import Nested from './Nested.svelte';
</script>

<p>This is a paragraph.</p>
<Nested />