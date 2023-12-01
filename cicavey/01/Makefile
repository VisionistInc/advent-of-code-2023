main:
	cargo run
wasm:
	wasm-pack build --features wee_alloc,console_error_panic_hook
	@echo -n 'let input=`' > www/input.txt
	@cat input.txt >> www/input.txt
	@echo -n '`;' >> www/input.txt
	cd www; npm install; npm start
clean:
	cargo clean
	rm -rf pkg
	rm -rf www/dist
	rm -rf www/node_modules