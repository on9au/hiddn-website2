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

sqlx-prepare:
ifeq ($(DATABASE_URL),)
	$(error "❌ DATABASE_URL is not set. Make sure it's defined in .env")
endif
	@echo "📦 Preparing SQLx offline schema..."
	sqlx prepare --check -- --bin hiddn-website

dev: $(env)
	make build-client
	rm -rf static && mkdir -p static && cp -r client/dist/* static
	RUST_LOG=debug SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --bin hiddn-website

preview: $(env)
	make build-client
	rm -rf static && mkdir -p static && cp -r client/dist/* static
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --release --bin hiddn-website

manager: $(env)
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --release --bin hiddn-cli

build: $(env) build-client sqlx-prepare
	cargo generate-lockfile
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo build --release --bin hiddn-website
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo build --release --bin hiddn-cli

docker-build: build
	@echo "🐳 Building Docker image..."
	sudo docker build \
		--build-arg SQLX_OFFLINE=$(SQLX_OFFLINE) \
		--build-arg DATABASE_URL=$(DATABASE_URL) \
		--build-arg VERSION=$(VER) \
		--tag on9au/hiddn-website:$(VER) .

docker-push: docker-build
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