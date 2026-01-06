# ----- Config -----
ENGINE_DIR := engine
WEB_DIR := web
WASM_OUT := $(WEB_DIR)/src/wasm
WASM_VERSION_FILE := $(WEB_DIR)/src/wasm_version.ts

WASM_PACK := wasm-pack
NPM := npm

.PHONY: help
help:
	@echo "Targets:"
	@echo "  make setup     - install web deps"
	@echo "  make wasm      - build Rust->Wasm into web/src/wasm"
	@echo "  make dev       - build wasm then run Vite dev server"
	@echo "  make watch     - watch Rust and rebuild wasm on changes"
	@echo "  make build     - production build"
	@echo "  make clean     - remove build artifacts"

.PHONY: setup
setup:
	cd $(WEB_DIR) && $(NPM) install

.PHONY: wasm
wasm:
	cd $(ENGINE_DIR) && $(WASM_PACK) build \
		--target web \
		--out-dir ../$(WASM_OUT) \
		--out-name engine
	@mkdir -p $(dir $(WASM_VERSION_FILE))
	@echo "export const WASM_BUILD_ID = \"$$(date +%s%3N)\";" > $(WASM_VERSION_FILE) 

.PHONY: dev
dev: wasm
	cd $(WEB_DIR) && $(NPM) run dev

.PHONY: watch
watch:
	cd $(ENGINE_DIR) && cargo watch \
		-w src -w Cargo.toml \
		-s "$(WASM_PACK) build --target web --out-dir ../$(WASM_OUT) --out-name engine && echo 'export const WASM_BUILD_ID = \"'$$(date +%s%3N)'\";' > ../$(WASM_VERSION_FILE)"

.PHONY: build
build: wasm
	cd $(WEB_DIR) && $(NPM) run build

.PHONY: clean
clean:
	rm -rf $(WASM_OUT)
	rm -rf $(WEB_DIR)/dist
	rm -rf $(WEB_DIR)/node_modules
