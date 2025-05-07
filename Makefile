env := .env
SQLX_OFFLINE ?= true
VER ?= dev


.PHONY: dev preview manager build npm-i docker-build docker-push clean generate-licenses

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

dev: $(env) ts-rs-gen generate-licenses
	make build-client
	rm -rf static && mkdir -p static && cp -r client/dist/* static
	cp NOTICE client/dist/NOTICE
	RUST_LOG=debug SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --bin hiddn-website

preview: $(env) ts-rs-gen generate-licenses
	make build-client
	rm -rf static && mkdir -p static && cp -r client/dist/* static
	cp NOTICE client/dist/NOTICE
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo run --release --bin hiddn-website

build: $(env) build-client sqlx-prepare ts-rs-gen generate-licenses
	cargo generate-lockfile
	cp NOTICE client/dist/NOTICE
	SQLX_OFFLINE=$(SQLX_OFFLINE) cargo build --release --bin hiddn-website

docker-build: build ts-rs-gen generate-licenses
	@echo "🐳 Building Docker image..."
	sudo docker build \
		--build-arg SQLX_OFFLINE=$(SQLX_OFFLINE) \
		--build-arg DATABASE_URL=$(DATABASE_URL) \
		--build-arg VERSION=$(VER) \
		--tag on9au/hiddn-website:$(VER) .

docker-push: docker-build ts-rs-gen generate-licenses
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

generate-licenses:
	@echo "📝 Generating LICENSES/NOTICE file..."

	# Generate Rust licenses
	@echo "🔄 Generating Rust license information..."
	cargo license --json > licenses.json

	# Generate JavaScript licenses
	@echo "🔄 Generating JavaScript license information..."
	cd client && license-checker --json > licenses.json

	# Combine Rust and JS licenses into a NOTICE file
	@echo "🔄 Combining Rust and JavaScript licenses into NOTICE..."
	@echo "This product includes third-party software components:" > NOTICE
	@echo "" >> NOTICE

	# Update this query based on actual JSON structure
	@echo "Rust dependencies:" >> NOTICE
	@cat licenses.json | jq -r '.[] | "- \(.name) \(.version) (\(.license))"' >> NOTICE
	@echo "" >> NOTICE

	# Update this query based on actual JSON structure
	@echo "JavaScript dependencies:" >> NOTICE
	@cat client/licenses.json | jq -r 'to_entries | .[] | "- \(.key) \(.value.licenses)"' >> NOTICE

	@echo "✅ NOTICE file generated!"

	# Now copy it to `static/NOTICE`
	@echo "📦 Copying NOTICE file to static directory..."
	mkdir -p static
	cp NOTICE static/NOTICE
	@echo "✅ NOTICE file copied to static directory!"
	@echo "📝 LICENSES/NOTICE file generation completed!"


