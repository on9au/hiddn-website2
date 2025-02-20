env := .env

$(env):
	echo "No .env file found, creating one..."
	echo "Please fill in the required values in the .env file"
	cp .env.example $(env)

typeshare:
	cargo install typeshare-cli

dev: $(env) typeshare
	cd client && npm i && npm run build && rm -rf ../static && mkdir ../static && cp -r ./dist/* ../static
	cargo build
	RUST_LOG=debug cargo run --bin hiddn-website

preview: $(env) typeshare
	cd client && npm i && npm run build && rm -rf ../static && mkdir ../static && cp -r ./dist/* ../static
	cargo build --release
	cargo run --release --bin hiddn-website

manager: $(env) typeshare
	cargo build --release
	cargo run --release --bin hiddn-cli

build: $(env) typeshare
	cd client && npm i && npm run build
	cargo generate-lockfile
	cargo build --release

npm i:
	cd client && npm i

docker-build: build
	docker build --tag=on9au/hiddn-website:$(VER) .

docker-push: docker-build
	docker push on9au/hiddn-website:$(VER)