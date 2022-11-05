all:

clean:
	git submodule foreach git checkout .

setup:
	for manifest in {font,opentype,postscript,truetype}/Cargo.toml; do \
		for crate in font opentype postscript truetype; do \
			cp "$${manifest}" /tmp/Cargo.toml && cat /tmp/Cargo.toml | \
			sed "s|^$${crate} = .*|$${crate} = { path = \"../$${crate}\" }|" > "$${manifest}"; \
		done \
	done

.PHONY: all clean setup
