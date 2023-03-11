default: all

all: deps build lint check test examples

auth:
	@echo "Copy and paste the following in the terminal where you"
	@echo "will be executing cargo commands:"
	@echo
	@echo '    eval $$(ssh-agent -s) && ssh-add'
	@echo

build:
	@cargo build

lint:
	@cargo +nightly clippy --version
	@cargo +nightly clippy --all-targets --all-features -- --no-deps -D clippy::all

cicd-lint:
	@cargo clippy --version
	@cargo clippy --all-targets --all-features -- --no-deps -D clippy::all

check:
	@cargo deny check
	@cargo +nightly udeps

test:
	@cargo test

deps:
	@cargo update

publish:
	@cargo publish

examples: example-simple example-file example-no-file example-no-file-defaults example-file-search \
example-file-struct example-files example-env example-env-str example-env-full

example-simple:
	@cargo run --example=simple

example-file:
	@cargo run --example=file

example-no-file:
	@cargo run --example=file_none

example-no-file-defaults:
	@cargo run --example=file_none_with_defaults

example-file-search:
	@cargo run --example=file_search

example-file-struct:
	@cargo run --example=file_struct

example-files:
	@cargo run --example=files

example-env:
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM=GCP \
	CFYG_SERVERS_DB_HOST="1.1.2.2" \
	CFYG_SERVERS_DB_NAME=db \
	CFYG_SERVERS_DB_USER=bob \
	CFYG_SERVERS_DB_MAX_CONNS=1250 \
	cargo run --example=env

example-env-str:
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM= \
	CFYG_SERVERS_DB_HOST="3.3.4.4" \
	CFYG_SERVERS_DB_NAME= \
	CFYG_SERVERS_DB_USER= \
	CFYG_SERVERS_DB_MAX_CONNS=1250 \
	cargo run --example=env_str

example-env-full:
	@CFYG_ENV=prod \
	CFYG_SERVERS_PLATFORM= \
	CFYG_SERVERS_DB_HOST="3.3.4.4" \
	CFYG_SERVERS_DB_NAME= \
	CFYG_SERVERS_DB_USER= \
	CFYG_SERVERS_DB_MAX_CONNS=1250 \
	cargo run --example=full

install-cargo-deny:
	@echo ">> Installing cargo deny ..."
	@cargo install --locked cargo-deny

setup-cargo-deny: install-cargo-deny
	@echo ">> Setting up cargo deny ..."
	@cargo deny init

install-udeps:
	@echo ">> Setting up cargo udeps ..."
	@cargo install cargo-udeps --locked

.PHONY: examples

nightly:
	@rustup toolchain install nightly
