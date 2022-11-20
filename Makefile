default: all

all: deps build test demos

auth:
	@echo "Copy and paste the following in the terminal where you"
	@echo "will be executing cargo commands:"
	@echo
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo

build:
	@cargo build

test:
	@cargo test

deps:
	@cargo update

publish:
	@cargo publish

demos:
	@cargo run --example=simple
	@cargo run --example=file
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM=GCP \
	CFYG_SERVERS_DB_HOST="1.1.2.2" \
	CFYG_SERVERS_DB_NAME=db \
	CFYG_SERVERS_DB_USER=bob \
	CFYG_SERVERS_DB_MAX_CONNS=1250 \
	cargo run --example=env
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM= \
	CFYG_SERVERS_DB_HOST="3.3.4.4" \
	CFYG_SERVERS_DB_NAME= \
	CFYG_SERVERS_DB_USER= \
	CFYG_SERVERS_DB_MAX_CONNS=1250 \
	cargo run --example=env_str
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM= \
	CFYG_SERVERS_DB_HOST="3.3.4.4" \
	CFYG_SERVERS_DB_NAME= \
	CFYG_SERVERS_DB_USER= \
	CFYG_SERVERS_DB_MAX_CONNS=1250 \
	cargo run --example=full
