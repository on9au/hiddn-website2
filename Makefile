env := .env

$(env):
	echo "No .env file found, creating one..."
	echo "Please fill in the required values in the .env file"
	cp .env.example $(env)

dev: $(env)
	cd client && npm i && npm run build
	RUST_LOG=debug cargo run

preview: $(env)
	cd client && npm i && npm run build
	cargo run --release

build: $(env)
	cd client && npm i && npm run build
	cargo build --release

npm i:
	cd client && npm i