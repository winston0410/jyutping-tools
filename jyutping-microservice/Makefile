include .env.local
# REF https://stackoverflow.com/questions/7507810/how-to-source-a-script-in-a-makefile
run:
	touch ./src/build.rs && cargo run
build:
	touch ./src/build.rs && cargo build
migrate:
	dropdb --if-exists -U postgres ${DATABASE_NAME} && \
	createdb -U postgres ${DATABASE_NAME} && \
	sqlx mig run
