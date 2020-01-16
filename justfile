dev := '--dev'

check: execk wasmck
build: exe wasm

execk:
	cd server && cargo check $([ x"{{dev}}" = x"--dev" ] || echo "--release") --target-dir=target
exe:
	cd server && cargo build $([ x"{{dev}}" = x"--dev" ] || echo "--release") --target-dir=target
server:
	cd server && cargo run $([ x"{{dev}}" = x"--dev" ] || echo "--release") --target-dir=target

wasmck:
	cd client && wasm-pack build {{dev}}
wasm:
	cd client && wasm-pack build {{dev}}
dist: wasm
	cd client/site && npm run build
client: dist
	sfz -r client/site/dist
npm:
	cd client/site && npm install
