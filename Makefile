all:
	cargo build

dev:
	DATABASE_URL=sqlite:./sqlite.db cargo run

# run before `make entities`  
migrate:
	touch sqlite.db
	DATABASE_URL=sqlite:./sqlite.db
	sea-orm-cli migrate up

entities:
	DATABASE_URL=sqlite:./sqlite.db sea-orm-cli generate entity -u sqlite:./sqlite.db -o src/entities

test:
	cargo test

cleandb:
	rm sqlite.db

health:
	curl -X 'GET' 'http://localhost:9090/api/health' -H 'accept: application/json'