//>> > - перенаправление потоков
> - перенаправление потоков stdin, stdout, stderr, они имеют номера 0,1,2 соответственно, когда создается процесс создаются эти три потока для него,например 
go run metadata/cmd/main.go 2>logs - переведет все ошибки из stderr в файл logs в папке в которой запускается процесс, или
go run metadata/cmd/main.go > logs 2>&1 - направить stdout в файл logs, stder туда же

| - перенаправляет stdout одной команды в stdin следующей, например
ps aux | grep docker | tail -n 2
