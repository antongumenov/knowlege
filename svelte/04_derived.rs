//>> ЗАВИСИМЫЕ ПЕРЕМЕННЫЕ
// зависимые прееменные задаются с помощью руны $derived

// елси обьявить реактивную переменную
let count = $state(1);

// можно обявить зависимую
let doubled = $derived(count * 2);

//>> СЛОЖНЫЙ РАСЧЕТ ЗАВИСИМОЙ ПЕРЕМЕННОЙ
// когда расчет зависимой переменной нужно положить в функцию
// использую руну $derived.by
<script>
	let numbers = $state([1, 2, 3]);
	let total = $derived.by(() => {
		let total = 0;
		for (const n of numbers) {
			total += n;
		}
		return total;
	});
</script>

//>> untrack
// вс ечто внутри $derived - считается его зависимостями
// и все это при изменении вызовет реакцию
// если я чтобы этого небыло для некоторых переменных
// я просто сую их в untrack
$effect(() => {
	// запустится при изменении `data`, но не при изменении `time`
	save(data, {
		timestamp: untrack(() => time)
	});
});