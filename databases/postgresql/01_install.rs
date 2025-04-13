//>> run in docker for dev
docker run --rm -p 5432:5432 -e POSTGRES_USER=ang -e POSTGRES_PASSWORD=1234 -e POSTGRES_DB=users_db -d postgres

//>> instalation
//<< install
// paru -S postgresql

//<< change user to postgress
// sudo -iu postgres

//<< init db
// set locale, encoding, data directory, use data page checksums
// initdb --locale=ru_RU.UTF-8 --encoding=UTF8 -D /var/lib/postgres/data --data-checksums
// exit

//<< start server
// sudo systemctl enable --now postgresql

//<< create db
// sudo -iu postgres
// createdb db_name

//<< create user
// createuser --interactive

//<< create db with new user
// createdb db_name -U user_name
