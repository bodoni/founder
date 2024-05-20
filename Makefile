RUST_BACKTRACE := full

.PHONY: all
all: test

.PHONY: test
test:

.PHONY: test-name-internal
test: test-name-internal
test-name-internal:
	cargo run --bin founder-name -- \
		--path tests/fixtures/internal
	rm -rf tests/fixtures/internal/name/*
	mv tests/fixtures/internal/fonts/*.txt tests/fixtures/internal/name
	[ "$$(git diff tests/fixtures/internal/name | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-name-external
test: test-name-external
test-name-external:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5724
	# https://github.com/google/fonts/issues/5805
	cargo run --bin founder-name -- \
		--path tests/fixtures/external \
		--exclude google-fonts/ofl/bungeecolor \
		--exclude google-fonts/ofl/bungeespice \
		--exclude google-fonts/ofl/gentiumbookbasic \
		--exclude google-fonts/ufl/ubuntu \
		--exclude web-platform-tests/css/WOFF2/support/SFNT-CFF-Fallback \
		--exclude web-platform-tests/css/WOFF2/support/SFNT-CFF-Reference \
		--exclude web-platform-tests/css/css-fonts/support/fonts/FontWithFancyFeatures \
		--exclude web-platform-tests/css/css-fonts/support/fonts/FontWithFeatures2 \
		--exclude web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup1 \
		--exclude web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup3 \
		--exclude web-platform-tests/fonts/CSSTest/csstest-basic-bold \
		--exclude web-platform-tests/fonts/CSSTest/csstest-basic-bolditalic \
		--exclude web-platform-tests/fonts/CSSTest/csstest-basic-regular \
		--workers "$$((4 * $$(nproc --all)))" \
		> /dev/null

.PHONY: test-rasterize-internal
test: test-rasterize-internal
test-rasterize-internal:
	cargo run --bin founder-rasterize -- \
		--path tests/fixtures/internal/vectorize \
		--exclude AdobeBlank
	rm tests/fixtures/internal/vectorize/**/*.png

.PHONY: test-vectorize-internal
test: test-vectorize-internal
test-vectorize-internal:
	rm -rf tests/fixtures/internal/vectorize
	cp -R tests/fixtures/internal/fonts tests/fixtures/internal/vectorize
	cargo run --bin founder-vectorize -- --path tests/fixtures/internal/vectorize
	rm tests/fixtures/internal/vectorize/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/internal/vectorize | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-vectorize-external
test: test-vectorize-external
test-vectorize-external:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5724
	cargo run --bin founder-vectorize -- \
		--path tests/fixtures/external \
		--exclude google-fonts/ofl/bungeecolor \
		--exclude google-fonts/ofl/bungeespice \
		--exclude google-fonts/ufl/ubuntu \
		--exclude web-platform-tests/css/WOFF2/support/SFNT-CFF-Fallback \
		--exclude web-platform-tests/css/WOFF2/support/SFNT-CFF-Reference \
		--exclude web-platform-tests/css/css-fonts/support/fonts/FontWithFancyFeatures \
		--exclude web-platform-tests/css/css-fonts/support/fonts/FontWithFeatures2 \
		--exclude web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup1 \
		--exclude web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup3 \
		--exclude web-platform-tests/css/css-writing-modes/support/WidthTest-Regular \
		--exclude web-platform-tests/css/css-writing-modes/support/tcu-font \
		--exclude web-platform-tests/fonts/adobe-fonts/CSSFWOrientationTest \
		--exclude web-platform-tests/fonts/adobe-fonts/CSSHWOrientationTest \
		--workers "$$((4 * $$(nproc --all)))" \
		> /dev/null
