//_ НОЖКИ
//~ GPIO
// в любой момент можно использовать ножки 

// на ввод и вывод
4, 14, 16(на wrover нельзя), 17(на wrover нельзя), 18, 19, 21, 22, 23, 25, 26, 27, 32,

// вывод
34, 35, 36, 39 

// ножки нельзя притягивать не к питанию не к земле во время старта
0, 2, 5, 12, 15,

// на все эти кнопки можно повесить орбработчики прерываний

//_ НАСТРОЙКА 
// на вход или на выход
// слабая подтяжка к 3,3в или к земле

// после перезагрузки состояние не гарантируется 
// по этому всегда нужно делать сброс
gpio_reset_pin(GPIO_NUM_2);

//~ РЕЖИМЫ
GPIO_MODE_DISABLE - порт отключен
GPIO_MODE_INPUT - только на вход
GPIO_MODE_OUTPUT - только на выход
GPIO_MODE_OUTPUT_OD - только на выход в режиме открытый сток
GPIO_MODE_INPUT_OUTPUT_OD - одновременно на вход и на выход в режиме открытый сток
GPIO_MODE_INPUT_OUTPUT - и на вход и на выход

gpio_set_direction(GPIO_NUM_2, GPIO_MODE_OUTPUT);

//~ РЕЖИМ ПОДТЯЖКИ
GPIO_PULLUP_ONLY – подтяжка к питанию +3,3В
GPIO_PULLDOWN_ONLY – подтяжка к “земле”
GPIO_PULLUP_PULLDOWN – подтяжка одновременно к питанию +3,3В и “земле”
GPIO_FLOATING – подтяжка отключена

gpio_set_pull_mode(button_gpio, GPIO_PULLUP_ONLY);

//~ ОГРАНИЧЕНИЕ ТОКА
// если подключаю диод
GPIO_DRIVE_CAP_0 – слабый, до ~5мА
GPIO_DRIVE_CAP_1 – сильнее, до ~10мА
GPIO_DRIVE_CAP_2 – средний (по умолчанию), до ~20мА
GPIO_DRIVE_CAP_3 – максимальный, до ~40мА

gpio_set_drive_capability(led_gpio, GPIO_DRIVE_CAP_0);