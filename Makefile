toolchain:
	@echo "Rust command-line utility versions:"
	rustc --version 			#rust compiler
	cargo --version 			#rust package manager
	rustfmt --version			#rust code formatter
	rustup --version			#rust toolchain manager
	clippy-driver --version		#rust linter

install:
	cargo clean &&\
	cargo build -j 1

evcxr:
	cargo install --locked evcxr_jupyter &&\
	evcxr_jupyter --install

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run 

release:
	cargo build --release

image:
	docker build -t rusty-ds .

rundocker:
	docker run -p 8080:8080 rusty-ds