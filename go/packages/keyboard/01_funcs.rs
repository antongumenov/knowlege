//>> НАЗНАЧЕНИЕ
// читает события нажатия клавишь в терминале

//<< ЧТЕНИЕ ПРОСТЫМ ЦИКЛОМ
if err := keyboard.Open(); err != nil {
    panic(err)
}
defer func() {
    _ = keyboard.Close()
}()

fmt.Println("Press ESC to quit")
for {
    char, key, err := keyboard.GetKey()
    if err != nil {
        panic(err)
    }
    fmt.Printf("You pressed: rune %q, key %X\r\n", char, key)
    if key == keyboard.KeyEsc {
        break
    }
}

//<< ЧТЕНИЕ С ПОМОЩЬЮ КАНАЛА
// получаю канал в который будут лететь события клавишь
keysEvents, err := keyboard.GetKeys(10)
if err != nil {
	panic(err)
}
defer func() {
	_ = keyboard.Close()
}()

fmt.Println("Press ESC to quit")
for {
	event := <-keysEvents
	if event.Err != nil {
		panic(event.Err)
	}
	fmt.Printf("You pressed: rune %q, key %X\r\n", event.Rune, event.Key)
	if event.Key == keyboard.KeyEsc {
		break
	}
}
