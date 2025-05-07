env := .env

.PHONY: dev preview manager build npm-i docker-build docker-push

$(env):
	@echo "No .env file found, creating one..."
	@cp .env.example $(env)
	@echo "Please fill in the required values in the .env file."

npm_install:
	cd client && [ -d node_modules ] || npm i

dev: $(env)
	cd client && npm i && npm run build
	rm -rf static && mkdir static && cp -r client/dist/* static
	RUST_LOG=debug cargo run --bin hiddn-website

preview: $(env)
	cd client && npm i && npm run build
	rm -rf static && mkdir static && cp -r client/dist/* static
	cargo run --release --bin hiddn-website

manager: $(env)
	cargo run --release --bin hiddn-cli

build: $(env)
	cd client && npm i && npm run build
	cargo generate-lockfile
	cargo build --release --bin hiddn-website
	cargo build --release --bin hiddn-cli

npm-i:
	cd client && npm i

docker-build: build
	sudo docker build --tag=on9au/hiddn-website:$(VER) .

docker-push: docker-build
	sudo docker push on9au/hiddn-website:$(VER)