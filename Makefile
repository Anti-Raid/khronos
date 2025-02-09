all:
	cargo build --release
with_experiments:
	cargo build --release --all-features