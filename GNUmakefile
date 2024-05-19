default: build
	cargo install --path .
build:
	cargo build -r
index: default
	nips > docs/index.html
-include cargo.mk
