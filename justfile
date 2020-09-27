build:
    @cargo build

lint:
    @cargo check
    @cargo clippy

release:
    @mkdir -p build
    @cargo build --release
    @cp ./target/release/tidy build/tidy

test:
    @cargo test
