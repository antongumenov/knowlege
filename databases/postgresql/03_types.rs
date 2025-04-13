
//>> integers
//! before you choose type to table field
//! look on your data and analize them
//! everytime choose smallest type for your data

//<< smalint or int2
// takes 2 bytes, in golang its int16

//<< iteger or int4
// takes 4 bytes, in golang its int32

//<< bigitn or int8
// takes 8 bytes, in golang its int64

//>> numeric - same as decimal
// for money
// but it's very very slow

//<< numeric without params 
// can hold any number, much more then bigint

//<< numeric with params
numeric(precision, scale)
// 14,88888
// precision=7(max number of unrounded digits)
// scale=5

// if i write
numeric(10,2)
// i can put into it
10.34334543
// and pg put 10.34
// but i cant put 
138383383.29
// it will be error 

// if i write
numeric(5,-2)
// i can put 7 digits
5 - unrounded
2 - rounded
//  i can put 
2344332
// and pg puts
2344300


