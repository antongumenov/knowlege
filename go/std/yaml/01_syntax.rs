//>> ЗАЧЕМ 
// чтобы писать и читать конфиги 

//>> ГДЕ ИСПОЛЬЗУЕТСЯ 
// swagger 
// docker compose 
// configs    

//>> СИНТАКСИС 

//<< # комнтарий  

//<< структура файла 
// файл должен начинаться с --- 
// и заканчиваться ...   
// файл состоит из блоков 

//<< блоки 
// блок жто как правила какая то переменая до : 
// далее ее значения могут быть и другие переменыые
// - это элемент списка  
---
list:
    - 10
    - some_string
    - "some_string"
...

//<< строки 
// строки могут быть записаны без ковычек 
// если пишем команды линукс то в ковычках 
// длинную строку можно записать в блочном формате c помощью >
// тут вся строка склеится в одну  
string: >
    asgsdfgsdfgsdf
    sdfgsdfgsdfgsdfgsd
    sdfgsdfgsdfgsdfgsdfg


// длинную строку можно записать в блочном формате c помощью |
// тут вся строка склеится c помощью преносов строки  
string: >
    asgsdfgsdfgsdf
    sdfgsdfgsdfgsdfgsd
    sdfgsdfgsdfgsdfgsdfg

//>> списки  
//<< простые 
// yes,  true, on, off, no,  false - булевые значения  
---
- 10
- строка
- yes
...

//<< именованные 
// десериализуются в массивы  
--- 
список_первго уровня:
    - 110
    - строка
    - true
    - списк_второго_уровня:
        - элемент 1
        - элемент 2
        - элемент 3
...

//<< словари 
// это наборы данных ключ значение 
// десериализуются в структуры  
---
User:
    Name: James
    Age: 37
    speciality: IT
...

//>> дополнения 
//<< нулевые значения 
// ~ - это nil 
// nul - это nil

//>> ПРИМЕРЫ 
//<< массив структур 
type Person struct {
	Name string `yaml:"name"`
	Age  int    `yaml:"age"`
}

type Config struct {
	People []Person `yaml:"people"`
}

// выглядит так  
people:
  - name: John
    age: 30
  - name: Alice
    age: 25

