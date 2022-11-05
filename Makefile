crates := font opentype postscript truetype

all: test

clean:
	git submodule foreach git checkout .

setup:
	for manifest in */Cargo.toml; do \
		for crate in ${crates}; do \
			cp "$${manifest}" /tmp/Cargo.toml && cat /tmp/Cargo.toml | \
			sed "s|^$${crate} = .*|$${crate} = { path = \"../$${crate}\" }|" > "$${manifest}"; \
		done \
	done

test:
	RUST_BACKTRACE=1 cargo run --bin scan -- --path tests/fixtures/google-fonts

.PHONY: all clean setup test
