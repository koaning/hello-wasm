install:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

compile:
	wasm-pack build --target web
