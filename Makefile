crates := font founder opentype postscript truetype typeface

all:

clean:
	git submodule foreach git checkout .

configure:
	for manifest in */Cargo.toml; do \
		for crate in ${crates}; do \
			cp "$${manifest}" /tmp/Cargo.toml && cat /tmp/Cargo.toml | \
			sed "s|^$${crate} = .*|$${crate} = { path = \"../$${crate}\" }|" > "$${manifest}"; \
		done \
	done

.PHONY: all clean configure
