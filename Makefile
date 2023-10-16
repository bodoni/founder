crates := font founder opentype postscript truetype typeface webtype

.PHONY: all
all:

.PHONY: clean
clean:
	for crate in ${crates}; do \
		pushd $${crate} > /dev/null; \
		git checkout .; \
		popd > /dev/null; \
	done

.PHONY: configure
configure:
	for manifest in */Cargo.toml; do \
		for crate in ${crates}; do \
			cp "$${manifest}" /tmp/Cargo.toml && cat /tmp/Cargo.toml | \
			sed "s|^$${crate} = .*|$${crate} = { path = \"../$${crate}\" }|" > "$${manifest}"; \
		done \
	done

.PHONY: update
update:
	for crate in ${crates}; do \
		pushd $${crate} > /dev/null; \
		git checkout main && git pull; \
		popd > /dev/null; \
	done
