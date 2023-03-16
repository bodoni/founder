crates := font founder opentype postscript truetype typeface webtype

all:

clean:
	for crate in ${crates}; do \
		pushd $${crate} > /dev/null; \
		git checkout .; \
		popd > /dev/null; \
	done

configure:
	for manifest in */Cargo.toml; do \
		for crate in ${crates}; do \
			cp "$${manifest}" /tmp/Cargo.toml && cat /tmp/Cargo.toml | \
			sed "s|^$${crate} = .*|$${crate} = { path = \"../$${crate}\" }|" > "$${manifest}"; \
		done \
	done

update:
	for crate in ${crates}; do \
		pushd $${crate} > /dev/null; \
		git checkout main && git pull; \
		popd > /dev/null; \
	done

.PHONY: all clean configure update
