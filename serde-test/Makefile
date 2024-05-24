export
OUT_NAME=gameplay

build:
	wasm-pack build --release --out-name $(OUT_NAME) --out-dir pkg

setup:
	RUST_BACKTRACE=FULL_TRACE ../target/debug/zkwasm-cli --params "./params" wasm_output setup --wasm pkg/gameplay_bg.wasm

dry-run: build setup
	sh dry-run.sh

clean:
	rm -rf pkg
