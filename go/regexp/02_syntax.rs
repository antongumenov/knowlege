//_ СИНТАКСИС

//~ указатели
// ^ - начало строки, ^абв - строка должна начаться с абв
// $ - окончание строки, текста, эюя$ - текст или строка должна закончиться эюя
// \A - в начале текста
// \b - конец ASCII слова - `\bcat\b` - так ищем слово cat если это отдельное слово
// \B - не конец ASCII слова - `\Bcat\b` - найдем cat в Blackcat

//~ метасимволы
// . - любой отдельный символ кроме новой строки, 
// | - или, кот|собака - кот или собака
// \ - следующий литерал а не символ выражения. \. - дальше будет именно точка а не любой символ

//~ классы символов
// [...] - любой из набора, [123] - 1 или 2 или 3, [А-Яа-я] - любая буква кирилици, в любом регистре   
// [^...] - любой кроме набора символов, [^0-9аол] - любой симвло кроме цифр, а, о, и л

//~ классы ASCII
// [[:alnum:]]  - цифра и буква в любом регистре (== [0-9A-Za-z])
// [[:alpha:]]  - буква в лбом регистре (== [A-Za-z])
// [[:ascii:]]  - ASCII символы (== [\x00-\x7F])
// [[:blank:]]  - табуляция и пробел (== [\t ])
// [[:cntrl:]]  - контрольные (== [\x00-\x1F\x7F])
// [[:digit:]]  - цифры (== [0-9])
// [[:graph:]]  - печатные, без пробелов и служебных (== [!-~] == [A-Za-z0-9!"#$%&'()*+,\-./:;<=>?@[\\\]^_`{|}~])
// [[:lower:]]  - буквы в нижнем регистре (== [a-z])
// [[:print:]]  - печатные плюс пробел (== [ -~] == [ [:graph:]])
// [[:punct:]]  - знаки припинания (== [!-/:-@[-`{-~]), вот эти: !"#$%&'()*+,\-./:;<=>?@[]^_`{|}
// [[:space:]]  - все пробельные символы (== [\t\n\v\f\r ])
// [[:upper:]]  - буквы в верхнем регистре (== [A-Z])
// [[:word:]]   - буквы, цифры и _ (== [0-9A-Za-z_])
// [[:xdigit:]] - цифры в шеснадцетиричной сс (== [0-9A-Fa-f])

//~ классы условных символов только ASCII
// \w - буква, цифра и _
// \W - не буква, цифра и _   
// \s - пробел, табуляция, разрыв строки
// \S - не пробел, табуляция, разрыв строки
// \d - цифра
// \D - не цифра

//~ Escape символы
// \a         - bell (== \007)
// \f         - form feed (== \014)
// \t         - таб (== \011)
// \n         - переход на новую строку (== \012)
// \r         - возврат коретки (== \015)
// \v         - вертикальный таб (== \013)
// \*         - literal *, for any punctuation character *
// \123       - восьмиричный код символа
// \x7F       - шеснадцатиричный код 2 цифры
// \x{10FFFF} - шеснадцатиричный код
// \Q...\E    - три точки

//~ группа
// позволяют поместить часть совпадения в отдельный массив
// квантификаторы после группы применяются ко всей группе
// группа 0 - это всегда все выражение
// () - группа
// (?P<name>re) - именованная группа
// (?:...) - исключить группу, "A (?:[A-z]*) for ([A-z]*) person" - здесь первая группа не будет в результатах 

//~ квантификаторы
// x* - ноль или больше x; предпочтительно больше, в строке xxxx найдет хххх 
// x*? - ноль или больше x; предпочтительно меньше, в строке xxxx найдет ничего
// x+ - один или больше x; предпочтительно больше, в строке xxxx найдет хххх
// x+? - один или больше x; предпочтительно меньше, в строке xxxx найдет х
// x? - ноль или один х, предпочтительно один, в строке xxxx найдет х
// x?? - ноль или один х, предпочтительно ноль, в строке xxxx найдет ничего
// x{n,m} - x от n до m, из строки хххх постарается вернуть х m раз
// x{n,m}? - x от n до m, из строки хххх постарается вернуть х n раз
// x{n,} - n или больше x, из строки хххх постарается вернуть х наибольшее количество раз
// x{n,}? - n или больше x, из строки хххх постарается вернуть х n раз
// x{n} - точно n x

//~ флаги
// флаги ставить так `(?ims)[а-я]+?`
// i - не обращать внимание на регистр
// m - позволяет ^ и $ соответствовать началу и концу строки в дополнении к началу и конце текста - многострочный режим
// s - позволяет точке соответствовать \n - однострочный режим

//~ символы которые нужно экранировать
// экранирую так /
// ^ $ * + ? { } [ ] \ | ( )

//~ символы Unicode
// если работаю с Unicode
// синтаксис символа Unicode такой
// \p{Cyrillic} 
// в {} ставлю либо класс языковоро семейства
// либо класс символа

//~ классы символов Unicode
// C    - oневидимые управляющие символы и неиспользуемые коды
// Cc   - правляющие ASCII или Latin-1 символы: 0x00–0x1F и 0x7F–0x9F
// Cf   - невидимый индикатор форматирования
// Co   - любой код, зарезервированный для частного использования
// Cs   - одна половина суррогатной пары в кодировке UTF-16

// L    - любая буква
// Ll   - буква в нижнем регистре
// Lm   - символ использующийся как буква
// Lo   - буква или идеограмма не имеющая строчных и прописных вариантов
// Lt   - диграф - звук из двух букв типа sh в английском
// Lu   - буква в верхнем регистре

// M    - mсимвол с другими символами
// Mc   - mсимвол с другими символами занимающий место типа символ гласных в восточных языках
// Me   - символ окружающий символ, круг квадрат
// Mn   - mсимвол с другими символами не занимающий места типа ударение

// N    - число любой письменности
// Nd   - десятичное число от 0 до 9
// Nl   - число как буква, например римская цифра
// No   - остальные, надстрочные и подстроячные, и не являющиеся 0-9

// P    - любой знак пунктуации
// Pc   - символ пунктуации, такой как подчёркивание, который соединяет слова.
// Pd   - любой вид дефиса или тире
// Pe   - любой вид закрывающей скобки
// Pf   - любой вид закрывающей кавычки
// Pi   - любой вид открывающей кавычки
// Ps   - любой вид открывающей скобки
// Po   - любой знак препинания, не относящийся к прошлым подкатегориям

// S    - математические символы, знаки валют, дингбаты, символы для рисования и т.д
// Sc   - символ валюты
// Sk   - обьединяющий символ
// Sm   - математический символ
// So   - символ не относящийся к другим

// Z    - любые разделители
// Zl   - разделитель строк
// Zp   - параграф
// Zs   - разделитель который не видим но занимает место

//~ классы языковых семейств Unicode
// Adlam
// Ahom
// Anatolian_Hieroglyphs
// Arabic
// Armenian
// Avestan
// Balinese
// Bamum
// Bassa_Vah
// Batak
// Bengali
// Bhaiksuki
// Bopomofo
// Brahmi
// Braille
// Buginese
// Buhid
// Canadian_Aboriginal
// Carian
// Caucasian_Albanian
// Chakma
// Cham
// Cherokee
// Chorasmian
// Common
// Coptic
// Cuneiform
// Cypriot
// Cypro_Minoan
// Cyrillic
// Deseret
// Devanagari
// Dives_Akuru
// Dogra
// Duployan
// Egyptian_Hieroglyphs
// Elbasan
// Elymaic
// Ethiopic
// Georgian
// Glagolitic
// Gothic
// Grantha
// Greek
// Gujarati
// Gunjala_Gondi
// Gurmukhi
// Han
// Hangul
// Hanifi_Rohingya
// Hanunoo
// Hatran
// Hebrew
// Hiragana
// Imperial_Aramaic
// Inherited
// Inscriptional_Pahlavi
// Inscriptional_Parthian
// Javanese
// Kaithi
// Kannada
// Katakana
// Kawi
// Kayah_Li
// Kharoshthi
// Khitan_Small_Script
// Khmer
// Khojki
// Khudawadi
// Lao
// Latin
// Lepcha
// Limbu
// Linear_A
// Linear_B
// Lisu
// Lycian
// Lydian
// Mahajani
// Makasar
// Malayalam
// Mandaic
// Manichaean
// Marchen
// Masaram_Gondi
// Medefaidrin
// Meetei_Mayek
// Mende_Kikakui
// Meroitic_Cursive
// Meroitic_Hieroglyphs
// Miao
// Modi
// Mongolian
// Mro
// Multani
// Myanmar
// Nabataean
// Nag_Mundari
// Nandinagari
// New_Tai_Lue
// Newa
// Nko
// Nushu
// Nyiakeng_Puachue_Hmong
// Ogham
// Ol_Chiki
// Old_Hungarian
// Old_Italic
// Old_North_Arabian
// Old_Permic
// Old_Persian
// Old_Sogdian
// Old_South_Arabian
// Old_Turkic
// Old_Uyghur
// Oriya
// Osage
// Osmanya
// Pahawh_Hmong
// Palmyrene
// Pau_Cin_Hau
// Phags_Pa
// Phoenician
// Psalter_Pahlavi
// Rejang
// Runic
// Samaritan
// Saurashtra
// Sharada
// Shavian
// Siddham
// SignWriting
// Sinhala
// Sogdian
// Sora_Sompeng
// Soyombo
// Sundanese
// Syloti_Nagri
// Syriac
// Tagalog
// Tagbanwa
// Tai_Le
// Tai_Tham
// Tai_Viet
// Takri
// Tamil
// Tangsa
// Tangut
// Telugu
// Thaana
// Thai
// Tibetan
// Tifinagh
// Tirhuta
// Toto
// Ugaritic
// Vai
// Vithkuqi
// Wancho
// Warang_Citi
// Yezidi
// Yi
// Zanabazar_Square
