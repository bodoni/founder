RUST_BACKTRACE := full

all: test

test: test-draw test-draw-selected
test: test-name test-name-selected

test-convert-selected:
	cargo run --bin founder-convert --features convert -- \
		--path assets/draw \
		--ignore AdobeBlank

test-draw:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	cargo run --bin founder-draw --features draw -- \
		--path ../tests/fixtures \
		--characters aiQW \
		--ignore google-fonts/ofl/bungeecolor \
		--ignore google-fonts/ofl/bungeespice \
		--ignore google-fonts/ofl/gruppo \
		--ignore google-fonts/ofl/iceland \
		--ignore google-fonts/ofl/kaushanscript \
		--ignore google-fonts/ufl/ubuntu \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Fallback \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Reference \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFancyFeatures \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFeatures2 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup1 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup3 \
		--ignore web-platform-tests/css/css-writing-modes/support/WidthTest-Regular \
		--ignore web-platform-tests/css/css-writing-modes/support/tcu-font \
		--ignore web-platform-tests/fonts/adobe-fonts/CSSFWOrientationTest \
		--ignore web-platform-tests/fonts/adobe-fonts/CSSHWOrientationTest \
		--workers 4 > /dev/null

test-draw-selected:
	cargo run --bin founder-draw --features draw -- \
		--path tests/fixtures \
		--characters aiQW \
		--mode free \
		--output assets/draw/free
	cargo run --bin founder-draw --features draw -- \
		--path tests/fixtures \
		--characters aiQW \
		--mode local \
		--output assets/draw/local
	cargo run --bin founder-draw --features draw -- \
		--path tests/fixtures \
		--characters aiQW \
		--mode global \
		--output assets/draw/global
	[ "$$(git diff assets/draw | wc -l | xargs)" = 0 ] || exit 1

test-name:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	# https://github.com/google/fonts/issues/5805
	cargo run --bin founder-name --features name -- \
		--path ../tests/fixtures \
		--ignore google-fonts/ofl/bungeecolor \
		--ignore google-fonts/ofl/bungeespice \
		--ignore google-fonts/ofl/gentiumbookbasic \
		--ignore google-fonts/ofl/gruppo \
		--ignore google-fonts/ofl/iceland \
		--ignore google-fonts/ofl/kaushanscript \
		--ignore google-fonts/ufl/ubuntu \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Fallback \
		--ignore web-platform-tests/css/WOFF2/support/SFNT-CFF-Reference \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFancyFeatures \
		--ignore web-platform-tests/css/css-fonts/support/fonts/FontWithFeatures2 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup1 \
		--ignore web-platform-tests/css/css-fonts/support/fonts/gsubtest-lookup3 \
		--ignore web-platform-tests/fonts/CSSTest/csstest-basic-bold \
		--ignore web-platform-tests/fonts/CSSTest/csstest-basic-bolditalic \
		--ignore web-platform-tests/fonts/CSSTest/csstest-basic-regular \
		--workers 4 > /dev/null

test-name-selected:
	cargo run --bin founder-name --features name -- \
		--path tests/fixtures \
		--output assets/name
	[ "$$(git diff assets/name | wc -l | xargs)" = 0 ] || exit 1

.PHONY: test
.PHONY: test-convert-selected
.PHONY: test-draw test-draw-selected
.PHONY: test-name test-name-selected
