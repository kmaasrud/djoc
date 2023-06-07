name = djoc_sandbox
PYTHON = python3
CARGO = cargo
WASM=public/$(name)_bg.wasm
JS=public/$(name).js
WASM_BUILD_DIR=pkg

SRC=$(shell find ./src ../src -name '*.rs')

all: $(WASM) $(JS)

$(WASM_BUILD_DIR): $(SRC)
	wasm-pack build --release --target web

$(WASM): $(WASM_BUILD_DIR)
	cp $(WASM_BUILD_DIR)/$(name)_bg.wasm $(WASM)

$(JS): $(WASM_BUILD_DIR)
	cp $(WASM_BUILD_DIR)/$(name).js $(JS)

run: $(WASM) $(JS)
	$(PYTHON) -m http.server -d public

clean:
	rm -rf pkg public/$(name)*
	$(CARGO) clean
