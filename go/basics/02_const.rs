//>> КОНСТАНТЫ
//<< создание типизированной константы
const price float32 = 275.00

//<< не типизированные
// могут принимать разные типы в зависимости от использования
// но только те типы в которые оно может быть приведено
// здесь может быть и int и float32 так как 2 можно представить как любой целочисленный тип
const price = 2

//<< определение нескольких констант
// так
const (
    pi float64 = 3.1415
    e float64 = 2.7182
)
//или так
const price, tax float32 = 275, 27.50

// или так
const quantity, inStock = 2, true

//>> IOTA и ENUM
// так в go создаются перечисления
const (
    Watersports = iota  // 0
    Soccer              // 1
    Chess               // 2
)