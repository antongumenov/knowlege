//_ INSTALATION
//~ INSTALL
// paru -S postgresql

//~ CHANGE USER TO POSTGRESS
// sudo -iu postgres

//~ INIT DB
// set locale, encoding, data directory, use data page checksums
// initdb --locale=ru_RU.UTF-8 --encoding=UTF8 -D /var/lib/postgres/data --data-checksums
// exit

//~ start server
// sudo systemctl enable --now postgresql

//~ create db
// sudo -iu postgres
// createdb db_name

//~ create user
// createuser --interactive

//~ create db with new user
// createdb db_name -U user_name
