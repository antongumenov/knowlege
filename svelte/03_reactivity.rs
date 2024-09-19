//_ РЕАКТИВНОСТЬ
// состояние автоматически привязано двусторонним связываниемb 
// и в зависимости от состояния можно реагировать как нужно
<script>
	let count = 0;

	function increment() {
		count+=1;
	}
</script>

<button>
	Clicked {count}
    // тернарный оператор
	{count === 1 ? 'time' : 'times'}
</button>

//_ ЗАВИСИМЫЕ ПЕРЕМЕННЫЕ
// елси одна переменная состояния зависит от другой 
// ее нужно определять так
// чтобы способность реактивно менять состояние сохранялась
let count = 0;
$: doubled = count * 2;

//_ ЛОВЛЯ СМЕНЫ СОСТОЯНИЯ
// я могу словить смену состояния 
// и сделать какое то действие
// то есть написать реакцию на смену состояния
let count = 0;
// так
$: {
	console.log(`the count is ${count}`);
	console.log(`this will also be logged whenever count changes`);
}

// или так
$: if (count >= 10) {
	alert("count is dangerously high!");
	count = 0;
}

//_ РЕАКТИВНОСТЬ МАССИВОВ И ОБЬЕКТОВ
// реактивность работает по присваиванию
// и она не работает при изменении массивов и обьектов
// чтобы реактивность работала для массивов и обьектов
// нужно после изменения сделать присваивание самому себе

// так 
function addNumber() {
	numbers.push(numbers.length + 1);
	numbers = numbers;
}
// или так
function addNumber() {
	numbers = [...numbers, numbers.length + 1];
}

// такое присвоение тоже сработает
function addNumber() {
	numbers[numbers.length] = numbers.length + 1;
}

// и все это сработает
const obj = { foo: { bar: 1 } };
const foo = obj.foo;
foo.bar = 2;    