dev:
	cd client && npm i && npm run build
	RUST_LOG=debug cargo run

preview:
	cd client && npm i && npm run build
	cargo run --release

build:
	cd client && npm i && npm run build
	cargo build

npm i:
	cd client && npm i