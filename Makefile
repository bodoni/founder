RUST_BACKTRACE := full

.PHONY: all
all: test

.PHONY: test
test:

.PHONY: test-internal-features
test: test-internal-features
test-internal-features:
	cargo run --bin founder-features -- \
		--path tests/fixtures/internal
	rm -rf tests/fixtures/internal/features/*
	mv tests/fixtures/internal/fonts/*.txt tests/fixtures/internal/features
	[ "$$(git diff tests/fixtures/internal/features | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-internal-names
test: test-internal-names
test-internal-names:
	cargo run --bin founder-names -- \
		--path tests/fixtures/internal
	rm -rf tests/fixtures/internal/names/*
	mv tests/fixtures/internal/fonts/*.txt tests/fixtures/internal/names
	[ "$$(git diff tests/fixtures/internal/names | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-internal-rasterize
test: test-internal-rasterize
test-internal-rasterize:
	cargo run --bin founder-rasterize -- \
		--path tests/fixtures/internal/vectorize \
		--exclude AdobeBlank
	rm tests/fixtures/internal/vectorize/**/*.png

.PHONY: test-internal-vectorize
test: test-internal-vectorize
test-internal-vectorize:
	rm -rf tests/fixtures/internal/vectorize
	cp -R tests/fixtures/internal/fonts tests/fixtures/internal/vectorize
	cargo run --bin founder-vectorize -- --path tests/fixtures/internal/vectorize
	rm tests/fixtures/internal/vectorize/*.otf
	rm tests/fixtures/internal/vectorize/*.ttf
	[ "$$(git diff tests/fixtures/internal/vectorize | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-external-names
test: test-external-names
test-external-names:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5724
	# https://github.com/google/fonts/issues/5805
	cargo run --bin founder-names -- \
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

.PHONY: test-external-vectorize
test: test-external-vectorize
test-external-vectorize:
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
		--exclude web-platform-tests/fonts/noto/cjk/NotoSansCJKjp-Regular-subset \
		--workers "$$((4 * $$(nproc --all)))" \
		> /dev/null
