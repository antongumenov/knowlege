//_ SLOT
// если обьявить компонент так
// Card.svelte
<div class="card">
	<slot></slot>
</div>

// в slot вставится все что угодно
// если я передам это между тегами компонента
<Card>
	<span>Patrick BATEMAN</span>
	<span>Vice President</span>
</Card>

//_ ИМЕНОВАННЫЕ SLOT
// иногда нужно разделить слоты компонента по назначению
// тогда я даю имена слотам 
// и раскладываю контент по имени
// Card.svelte
<div class="card">
	<header>
		<slot name="telephone"></slot>
		<slot name="company"></slot>
	</header>

	<slot></slot>

	<footer>
		<slot name="address"></slot>
	</footer>
</div>

// в каждый слот я могу положить что то определенное


<Card>
    // это попадет в слот по умолчанию
    <span>Patrick BATEMAN</span>
    <span>Vice President</span>

    // остальное по имени
    <span slot="telephone">212 555 6342</span>

    <span slot="company">
        Pierce &amp; Pierce
        <small>Mergers and Aquisitions</small>
    </span>

    <span slot="address">358 Exchange Place, New York, N.Y. 10099 fax 212 555 6390 telex 10 4534</span>
</Card>

//_ ЗНАЧЕНИЯ СЛОТА ПО УМОЛЧАНИЮ
// в слот можно засунуть значения по умолчанию
<slot name="telephone">
	<i>Какой нибудь контент</i>
</slot>

//_ ПРОПСЫ В СЛОТ
// когда я делаю компонент
// и у меня в нем есть данные
// пусть я хочу сделать отображение этих нданный переменчивым
// тогда я могу описать слот 
// и передать в него параметры
// и при добавлении этого компонента выше
// я могу поймать их и отрисовать контент слота как мне нужно

//~ пусть есть компонент NameList
<script>
    const names = ["Sam", "John", "Peter"]
</script>

{#each names as firstname(firstname)}
    // прокидываю параметр в слот так
    <slot name="name" firstname={firstname}></slot>
    // или так если имена совпалдают
    <slot name="name" {firstname}></slot>
{/each}

//~ принимаю в слот
<NameList>
    // ловлю параметр и вывожу в слолте так как я хочу
    <h1 slot="name" let:firstname> My friend {firstname}</h1>
</NameList>

//~ можно и так
// в компоненте передаю пропсы
<script>
    const names = ["Sam", "John", "Peter"]
</script>

{#each names as firstname(firstname)}
    // прокидываю параметр в слот так
    <slot {firstname}></slot>
{/each}

//~ принимаю в слот
<NameList let:firstname = {name}>// как бы псевдоним, на самом деле просто привязываю переменные
    // ловлю параметр и вывожу в слолте так как я хочу
    <h1 > My friend {name}</h1>
</NameList>

//_ ПРОВЕРКА КОНТЕНТА В СЛОТЕ
// иногда я не хочу передавать данные в какой нибудь именованный слот
// но если он у меня описан, он выведется(например покажутся рамки)
// тогда мне нужно проверить есть ли там контент
// и если нет, слот провто не бьудет отображен
{#if $$slots.header}
	<div class="header">
		<slot name="header"></slot>
	</div>
{/if}