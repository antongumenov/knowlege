//>> УСТАНОВКА ПРИЛОЖЕНИЙ

//<<  собираю приложение

//<< в папке проекта создаю иконку с имененем типа app_name.png

//<< создаю файл типа app_name.desktop
[Desktop Entry]

# Тип ярлыка
Type=Application

# Версия спецификации ярлыков приложений, которой соответствует этот файл
Version=1.0

# Название приложения
Name=Tong Sim

# Комментарий, который может/будет использоваться в качестве подсказки
Comment=Hydravlic Tong Simulator

# Путь к папке, в которой выполняется исполняемый файл
Path=/store/dev/projects/tong_sim

# Исполняемый файл приложения, возможно с аргументами.
Exec=tong-sim

# Имя значка, который будет использоваться для отображения этого ярлыка.
Icon=tsIcon

# Описывает, должно ли это приложение запускаться в терминале или нет
Terminal=false

# Описывает категории, в которых должна отображаться этот ярлык
Categories=Education;

//<< проверяю на валидность
desktop-file-validate app_name.desktop

//<< устанвливаю в систему
// второй путь это место нахождения программы
// можно передожить куда угодно
desktop-file-install --dir=$HOME/.local/share/applications /store/dev/projects/tong_sim/app_name.desktop    

//<< перемещаю иконку в папку для иконок
sudo mv app_name.png /usr/share/pixmaps

//<< обновляю базу данных приложений
// для того пути куда установил типа приложение
update-desktop-database ~/.local/share/applications 

//<< ну и можно перезагрузить систему