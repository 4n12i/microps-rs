commit: 
	make check
	git add .
	git commit
	
run: 
	make check
	cargo run
	
build: 
	make check
	cargo build

check:
	cargo fmt
	cargo clippy -- -D warnings
	cargo check