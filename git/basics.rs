//>> Первоначальная настройка
git config --global user.name "[имя]"
git config --global user.email "[адрес электронной почты]"

//>> Создание репозитория
git init [название проекта] - создание в папке
git clone [url-адрес] - клонирование из репозитория

//>> Внесение изменений
git status - перечисляет все что нужно комитить
git diff - различия по внесённым изменениям в ещё не проиндексированных файлах
git add [файл] - индексирует файл для последующего коммита
git diff --staged - различия между проиндексированной и последней зафиксированной версиями файлов
git reset [файл] - отменяет индексацию указанного файла, при этом сохраняет его содержимое
git commit -m "[сообщение с описанием]" -фиксирует проиндексированные изменения и сохраняет их в историю версий
git commit --amend -m "Updated message for the previous commit" - изменить сообщение комита если написал не то
git commit --amend --no-edit - правка текущего коммита, без создания нового коммита, если че то забыл добавить

//>> Коллективная работа
git branch - все ветки
git checkout -а - удаленные ветки
git checkout [имя ветки] - переход на ветку
git branch [имя ветки] - cоздаёт новую ветку
git branch -m master main - переименовать ветку из master в main
git checkout -b new_branch_name - создание ветки и пеерход на нее автоматически(-b)
git switch -c [имя ветки] - переключается на выбранную ветку и обновляет рабочую директорию до её состояния
git merge [имя ветки] - вносит изменения указанной ветки в текущую ветку
git branch -d [имя ветки] - удаляет выбранную ветку, работает только после слияния с основной
git branch -D [имя ветки] - удаляет выбранную ветку принудительно
git merge --no-ff existing_branch_name - слияние с созданием коммита
git merge --abort - прервать слияние
git reset - восстановить все файлы после конфликта слияния

//>> Просмотр истории
git log - история коммитов для текущей ветки (-p - флаг для подробного изучения изменений в каждый файл)
git log --follow [файл] - история изменений конкретного файла, включая его переименование
git diff [первая ветка]...[вторая ветка] - показывает разницу между содержанием коммитов двух веток
git show [коммит] - ыводит информацию и показывает изменения в выбранном коммите
git log --graph --oneline --decorate - просмотр коммитов в виде графика в текущей ветке
git log --all --graph --oneline --decorate - просмотр коммитов в виде графика во всех ветках

//>> Откат коммитов
git reset [коммит] - oтменяет все коммиты после заданного, оставляя все изменения в рабочей директории
git reset --hard [коммит] - сбрасывает всю историю вместе с состоянием рабочей директории до указанного коммита.
git reset HEAD somefile.js - восстановить файл подготовленный к коммиту
git reset HEAD somefile.js - восстановить все файлы подготовленные к коммиту
git checkout somefile.js - восстановить файл не подготовленный к коммиту 
git revert HEAD - откатит последний коммит, создав новый коммит, не удаляя состояние последнего коммита
git revert 1af17e - откат конкретного коммита, создастся новый комит, не удаляя состояние откатываемого коммита
git reset --soft head~3 - удалить три первых коммита из истории 

//>> Синхронизация с удалённым репозиторием 
git remote -v - удаленные репозитории
git remote add origin https://github.com/antongum/microservices.git - добавление удаленного репозитория
git push -u origin main - загружает все изменения локальной ветки в удалённый репозиторий
git remote show origin - сведенья об удаленном репозитории
git fetch [удалённый репозиторий] - cкачивает всю историю из удалённого репозитория
git merge [удалённый репозиторий]/[ветка] - вносит изменения из ветки удалённого репозитория в текущую ветку локального репозитория
git push [удалённый репозиторий] [ветка] - загружает все изменения локальной ветки в удалённый репозиторий
git pull - загружает историю из удалённого репозитория и объединяет её с локальной. pull = fetch + merge
git push origin --delete existing_branch_name - удаление ветки в удаленном репозитории
git merge origin - слияние удаленного с локальным
git push -u origin new_branch - отправка новой ветки в удаленный репозиторий
git remote -v - с каким удаленным репозиторием работаю

//>> Игнорирование некоторых файлов
Исключение временных и вторичных файлов и директорий пишем в .gitignore так

*.log  - все файлы с расширением log
build/ - все что в папке build 
temp-* - все что начинается с temp-

git ls-files --others --ignored --exclude-standard - список всех игнорируемых файлов в текущем проекте

//>> Сохранение фрагментов
git stash - временно сохраняет все незафиксированные изменения отслеживаемых файлов
git stash pop - восстанавливает состояние ранее сохранённых версий файлов
git stash list - выводит список всех временных сохранений
git stash drop - сбрасывает последние временно сохранённыe изменения

//>> Теги
Отмечают версии тегами, чтобы было понятно, как правило самые важные коммиты(задаем только в main)
git tag - какие теги есть
git tag v1.0.0 - установить ветку в текущем коммите(делаю сразу после коммита)
git push origin v1.0.0 - закинуть тег на текущий коммит в удаленный репозиторий
git push origin --tags - запушить все незапушенные теги 
git tag -d v1.0.0 - удаление тега локально
git tag v1.0.0 54651316 - добавить тег на конкретный коммит
git push origin --delete v1.0.0 - удаляю с удаленного репозитория
git checkout v1.0.0 - переход на комит с тегом v1.0.0
git checkout main - вернуться обратно

//>> SSH
Нужно для безовасного клонирования гит репозиториев так как http не безопасен
ssh-keygen -t ed25519 -b 4096 - генерирую ключ
Сгенерируется ключ в файл /home/ang/.ssh/ed25519.pub
Запускаю ssh-agent
eval "$(ssh-agent -s)"
Добавляю ранее созданный ключ:
ssh-add ~/.ssh/id_ed25519
Копирую ключ из файла ed25519.pub 
Добавляю в настройки профиля ssh на GitHub

//>> Переходы
git checkout 54161641651ljbafsdf6156 - переход на коммит по номеру
git checkout v1.0.0 - переход на комит с тегом v1.0.0
git checkout main - вернуться к последнему коммиту

