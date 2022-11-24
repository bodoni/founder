crates := font opentype postscript truetype

all: tests

clean:
	git submodule foreach git checkout .

setup:
	for manifest in */Cargo.toml; do \
		for crate in ${crates}; do \
			cp "$${manifest}" /tmp/Cargo.toml && cat /tmp/Cargo.toml | \
			sed "s|^$${crate} = .*|$${crate} = { path = \"../$${crate}\" }|" > "$${manifest}"; \
		done \
	done

tests:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5620
	$(MAKE) -C tests/fixtures
	RUST_BACKTRACE=1 cargo run --bin scan -- \
		--path tests/fixtures \
		--ignore Noto_Sans_JP \
		--ignore gruppo \
		--ignore iceland \
		--ignore kaushanscript \
		--ignore ubuntu

.PHONY: all clean setup tests
