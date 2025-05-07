env := .env
SQLX_OFFLINE ?= true
VER ?= dev


.PHONY: dev preview manager build npm-i docker-build docker-push

$(env):
	@echo "🔧 No .env file found, creating one..."
	@cp .env.example $(env)
	@echo "✏️  Please fill in the required values in the .env file."

build-client:
	cd client && npm install && npm run build
	@echo "🔨 Client build completed." 

ts-rs-gen:
	@echo "🔄 Generating TypeScript to Rust bindings..."
	cargo test export_bindings
	@echo "🔄 TypeScript to Rust bindings generated."

sqlx-prepare:
ifeq ($(DATABASE_URL),)
	$(error "❌ DATABASE_URL is not set. Make sure it's defined in .env")
endif
	@echo "📦 Preparing SQLx offline schema..."
	sqlx prepare --check -- --bin hiddn-website

dev: $(env) ts-rs-gen
	make build-client
	rm -rf static && mkdir -p static && cp -r client/dist/* static
	RUST_LOG=debug SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --bin hiddn-website

preview: $(env) ts-rs-gen
	make build-client
	rm -rf static && mkdir -p static && cp -r client/dist/* static
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --release --bin hiddn-website

manager: $(env) ts-rs-gen
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --release --bin hiddn-cli

build: $(env) build-client sqlx-prepare ts-rs-gen
	cargo generate-lockfile
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo build --release --bin hiddn-website
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo build --release --bin hiddn-cli

docker-build: build ts-rs-gen
	@echo "🐳 Building Docker image..."
	sudo docker build \
		--build-arg SQLX_OFFLINE=$(SQLX_OFFLINE) \
		--build-arg DATABASE_URL=$(DATABASE_URL) \
		--build-arg VERSION=$(VER) \
		--tag on9au/hiddn-website:$(VER) .

docker-push: docker-build ts-rs-gen
	@echo "📤 Pushing Docker image..."
	sudo docker push on9au/hiddn-website:$(VER)

clean: 
	@echo "🧹 Cleaning up..."
	rm -rf static
	rm -rf client/dist
	rm -rf client/node_modules
	rm -rf client/.env.local
	rm -rf client/.env.production.local
	cargo clean
	@echo "🧼 Cleaned up."

npm-i:
	cd client && npm i