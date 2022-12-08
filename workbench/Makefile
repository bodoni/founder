all: tests

tests: tests-draw tests-scan tests-sign

tests-draw:
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/SourceSerifPro-Regular.otf \
		--glyph "&" > assets/glyph-otf.svg
	cargo run --bin draw --features drawing --quiet -- \
		--font ../font/tests/fixtures/OpenSans-Italic.ttf \
		--glyph "&" > assets/glyph-ttf.svg
	[ "$$(git diff assets | wc -l | xargs)" == 0 ] || exit 1

tests-scan:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	$(MAKE) -C tests/fixtures
	RUST_BACKTRACE=full cargo run --bin scan --features scanning -- \
		--path tests/fixtures \
		--ignore gruppo \
		--ignore iceland \
		--ignore kaushanscript \
		--ignore ubuntu \
		--workers 4

tests-sign:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	$(MAKE) -C tests/fixtures
	RUST_BACKTRACE=full cargo run --bin sign --features drawing,scanning -- \
		--path tests/fixtures \
		--ignore gruppo \
		--ignore iceland \
		--ignore kaushanscript \
		--ignore ubuntu \
		--workers 4

.PHONY: tests tests-scan
