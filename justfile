# default recipe. Run formatter, lint and test
check: 
    cargo fmt 
    cargo clippy -- -D warnings 
    cargo check
    cargo test

# check and run 
run: 
    just check
    cargo run

# check and build
build: 
    just check 
    cargo build
    
# create a commit
commit: 
    just check 
    git add .
    git commit
