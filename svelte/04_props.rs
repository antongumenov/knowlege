//_ PROPS
// отправляю в компонент так
<script>
	import Nested from './Nested.svelte';
</script>

<Nested answer={42} />

// принимаю так
<script>
	export let answer;
</script>

// если пропс не передан
// можно указать значение по умолчанию
<script>
	export let answer = "a mystery";
</script>

//_ ДЕССТРУКТУРИЗАЦИЯ ОБЬЕКТОВ ПРОПСОВ
// если пропсы принимаются как переменные
// обьект при передаче можно деструктуризировать
<script>
	import PackageInfo from './PackageInfo.svelte';

	const pkg = {
		name: 'svelte',
		speed: 'blazing',
		version: 4,
		website: 'https://svelte.dev'
	};
</script>

// так 
<PackageInfo {...pkg} />

// вместо этого
<PackageInfo
	name={pkg.name}
	speed={pkg.speed}
	version={pkg.version}
	website={pkg.website}
/>