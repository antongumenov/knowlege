//_ ФОРМАТИРОВАНИЕ СТРОКИ ДАТЫ
// дата в шаблоне должна быть 2 января 2006 года 15:04
layout := "Day: 02 Month: Jan Year: 2006"
timeNow := time.Now()
fmt.Println(timeNow.Format(layout)) // Day: 04 Month: Aug Year: 2024

//~ встроенные шаблоны
// 	ANSIC			Mon Jan _2 15:04:05 2006
// 	UnixDate		Mon Jan _2 15:04:05 MST 2006
// 	RubyDate		Mon Jan 02 15:04:05 -0700 2006
// 	RFC822			02 Jan 06 15:04 MST
// 	RFC822Z			02 Jan 06 15:04 -0700
// 	RFC850			Monday, 02-Jan-06 15:04:05 MST
// 	RFC1123			Mon, 02 Jan 2006 15:04:05 MST
// 	RFC1123Z		Mon, 02 Jan 2006 15:04:05 -0700
// 	RFC3339			2006-01-02T15:04:05Z07:00
// 	RFC3339Nano		2006-01-02T15:04:05.999999999Z07:00
// 	Kitchen			3:04PM
// 	Stamp			Jan _2 15:04:05
// 	StampMilli		Jan _2 15:04:05.000
// 	StampMicro		Jan _2 15:04:05.000000
// 	StampNano		Jan _2 15:04:05.000000000

fmt.Println(timeNow.Format(time.Kitchen)) // 5:44PM

//_ РАЗБОР ЗНАЧЕНИЙ ВРЕМЕНИ ИЗ СТРОК
//~ Parse(layout, str)
// парсит строку по шаблону
layout = "2006-Jan-02"
d := "2015-Jun-02"
timeParsed, _ := time.Parse(layout, d)
fmt.Println(timeParsed) // 2015-06-02 00:00:00 +0000 UTC

//~ ParseInLocation(layout, str, location)
// парсит строку по шаблону и локации
london, _ := time.LoadLocation("Europe/London")
timeParsed, _ = time.ParseInLocation(layout, d, london)
fmt.Println(timeParsed) // 2015-06-02 00:00:00 +0100 BST