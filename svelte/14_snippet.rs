//_ ВЫДЕЛЕНИЕ ШАБЛОНА В СНИППЕТ
// полезно когда один и тот же шаблон 
// рендериться по условию или в each
//~ синтаксис
{#snippet name(param1, param2, paramN)}...{/snippet}

//~ пример
{#snippet sum(a, b)}
	<p>{a} + {b} = {a + b}</p>
{/snippet}

// отрендерить три раза
{@render sum(1, 2)}
{@render sum(3, 4)}
{@render sum(5, 6)}