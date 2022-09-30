all:
	cargo build

dev:
	DATABASE_URL=sqlite:./sqlite.db cargo run

# run before `make entities`  
migrate:
	sea migrate up

entities:
	sea-orm-cli generate entity -o src/entities

test:
	cargo test

cleandb:
	rm sqlite.db

db:
	touch sqlite.db
	DATABASE_URL=sqlite:./sqlite.db make migrate
	DATABASE_URL=sqlite:./sqlite.db make entities

health:
	curl -X 'GET' 'http://localhost:9090/api/health' -H 'accept: application/json'