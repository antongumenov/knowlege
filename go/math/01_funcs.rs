//_ ОКРУГЛЕНИЕ ЗНАЧЕНИЙ
// не меняет тип
f := 15.55

//~ округление до большего
fmt.Println(math.Ceil(f)) // 16

//~ округление до меньшего
fmt.Println(math.Floor(f)) // 15

//~ округление до ближайшего
fmt.Println(math.Round(f)) // 15
