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
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM=GCP \
	CFYG_SERVERSDB_HOST="1.1.2.2" \
	CFYG_SERVERSDB_NAME=db \
	CFYG_SERVERSDB_USER=bob \
	CFYG_SERVERSDB_MAX_CONNS=1250 \
	cargo run --example=env
	@cargo run --example=full
