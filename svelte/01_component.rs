//>> КОМПОНЕНТ
// этот блок создается только один раз при  создании одного компонента
// переменные обьявленные здесь будут доступны в любом экземпляре
// но не являются реактивными
// сюда можно сувать импорты и общие данные
<script module>
	let age = 15;
</script>

// состояние отдельного компонента и логика живет здесь
// здесь можно использовать общие данные из <script module>
<script>
	let name = 'Svelte' + age;
</script>

// так состояние подключается в html динамически
// и автоматически поривязано к состоянию
// в скобках можно писать код
<h1>Hello {name.toUpperCase()}!</h1>

// здесь пишу стили
<style>
	p {
		font-size: 2em;
	}
</style>

//>> ВСТРАИВАНИЕ КОМПОНЕНТОВ ИЗ ДРУГИХ ФАЙЛОВ

<script module>
	import Nested from './Nested.svelte';
</script>

<p>This is a paragraph.</p>
<Nested />