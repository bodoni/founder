RUST_BACKTRACE := full

.PHONY: all
all: test

.PHONY: test
test:

.PHONY: test-convert-selected
test: test-convert-selected
test-convert-selected:
	cargo run --bin founder-convert --features convert -- \
		--path tests/fixtures/draw \
		--exclude AdobeBlank

.PHONY: test-draw
test: test-draw
test-draw:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	cargo run --bin founder-draw --features draw -- \
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

.PHONY: test-draw-selected-free
test: test-draw-selected-free
test-draw-selected-free:
	rm -rf tests/fixtures/draw/free
	cp -R tests/fixtures/fonts tests/fixtures/draw/free
	cargo run --bin founder-draw --features draw -- --path tests/fixtures/draw/free --mode free
	rm tests/fixtures/draw/free/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/draw/free | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-draw-selected-local
test: test-draw-selected-local
test-draw-selected-local:
	rm -rf tests/fixtures/draw/local
	cp -R tests/fixtures/fonts tests/fixtures/draw/local
	cargo run --bin founder-draw --features draw -- --path tests/fixtures/draw/local --mode local
	rm tests/fixtures/draw/local/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/draw/local | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test-draw-selected-global
test: test-draw-selected-global
test-draw-selected-global:
	rm -rf tests/fixtures/draw/global
	cp -R tests/fixtures/fonts tests/fixtures/draw/global
	cargo run --bin founder-draw --features draw -- --path tests/fixtures/draw/global --mode global
	rm tests/fixtures/draw/global/*.{otf,ttf}
	[ "$$(git diff tests/fixtures/draw/global | wc -l | xargs)" = 0 ] || exit 1

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
