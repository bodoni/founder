RUST_BACKTRACE := full

.PHONY: all
all: test

.PHONY: test
test:

.PHONY: test-name
test: test-name
test-name:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	# https://github.com/google/fonts/issues/5805
	cargo run --bin founder-name --features name -- \
		--path ../tests/fixtures/fonts \
		--exclude google-fonts/ofl/bungeecolor \
		--exclude google-fonts/ofl/bungeespice \
		--exclude google-fonts/ofl/gentiumbookbasic \
		--exclude google-fonts/ofl/gruppo \
		--exclude google-fonts/ofl/iceland \
		--exclude google-fonts/ofl/kaushanscript \
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

.PHONY: test-name-selected
test: test-name-selected
test-name-selected:
	cargo run --bin founder-name --features name -- \
		--path tests/fixtures/fonts
	rm -rf tests/fixtures/name/*
	mv tests/fixtures/fonts/*.txt tests/fixtures/name
	[ "$$(git diff tests/fixtures/name | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-rasterize-selected
test: test-rasterize-selected
test-rasterize-selected:
	cargo run --bin founder-rasterize --features rasterize -- \
		--path tests/fixtures/vectorize \
		--exclude AdobeBlank

.PHONY: test-vectorize
test: test-vectorize
test-vectorize:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	cargo run --bin founder-vectorize --features vectorize -- \
		--path ../tests/fixtures/fonts \
		--exclude google-fonts/ofl/bungeecolor \
		--exclude google-fonts/ofl/bungeespice \
		--exclude google-fonts/ofl/gruppo \
		--exclude google-fonts/ofl/iceland \
		--exclude google-fonts/ofl/kaushanscript \
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

.PHONY: test-vectorize-selected-free
test: test-vectorize-selected-free
test-vectorize-selected-free:
	rm -rf tests/fixtures/vectorize/free
	cp -R tests/fixtures/fonts tests/fixtures/vectorize/free
	cargo run --bin founder-vectorize --features vectorize -- --path tests/fixtures/vectorize/free --mode free
	rm tests/fixtures/vectorize/free/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/vectorize/free | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-vectorize-selected-local
test: test-vectorize-selected-local
test-vectorize-selected-local:
	rm -rf tests/fixtures/vectorize/local
	cp -R tests/fixtures/fonts tests/fixtures/vectorize/local
	cargo run --bin founder-vectorize --features vectorize -- --path tests/fixtures/vectorize/local --mode local
	rm tests/fixtures/vectorize/local/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/vectorize/local | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-vectorize-selected-global
test: test-vectorize-selected-global
test-vectorize-selected-global:
	rm -rf tests/fixtures/vectorize/global
	cp -R tests/fixtures/fonts tests/fixtures/vectorize/global
	cargo run --bin founder-vectorize --features vectorize -- --path tests/fixtures/vectorize/global --mode global
	rm tests/fixtures/vectorize/global/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/vectorize/global | wc -l | xargs)" = 0 ] || exit 1
