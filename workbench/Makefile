all: tests

tests: tests-draw tests-scan tests-sign

tests-draw:
	RUST_BACKTRACE=full cargo run --bin draw --features drawing -- \
		--font ../font/tests/fixtures/SourceSerifPro-Regular.otf \
		--glyph "&"
	RUST_BACKTRACE=full cargo run --bin draw --features drawing -- \
		--font ../font/tests/fixtures/OpenSans-Italic.ttf \
		--glyph "&"

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
	$(MAKE) -C tests/fixtures
	RUST_BACKTRACE=full cargo run --bin sign --features drawing,scanning -- \
		--path tests/fixtures \
		--ignore gruppo \
		--ignore iceland \
		--ignore kaushanscript \
		--ignore ubuntu \
		--workers 4

.PHONY: tests tests-scan
