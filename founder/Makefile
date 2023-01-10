RUST_BACKTRACE := full

all: tests

tests: tests-draw-selected tests-name-selected tests-sign-selected
tests: tests-scan tests-sign

tests-draw-selected:
	cargo run --bin founder-draw -- \
		--path tests/fixtures \
		--character a \
		--output assets/draw
	[ "$$(git diff assets/draw | wc -l | xargs)" = 0 ] || exit 1

tests-name-selected:
	cargo run --bin founder-name -- \
		--path tests/fixtures \
		--output assets/name
	[ "$$(git diff assets/name | wc -l | xargs)" = 0 ] || exit 1

tests-sign-selected:
	cargo run --bin founder-sign -- \
		--path tests/fixtures \
		--characters anop \
		--output assets/sign
	[ "$$(git diff assets/sign | wc -l | xargs)" = 0 ] || exit 1

tests-scan:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	cargo run --bin founder-scan -- \
		--path ../tests/fixtures \
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
		--workers 4

tests-sign:
	# https://github.com/google/fonts/issues/5551
	# https://github.com/google/fonts/issues/5553
	# https://github.com/google/fonts/issues/5724
	cargo run --bin founder-sign -- \
		--path ../tests/fixtures \
		--output assets/sign \
		--characters anop \
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
		--ignore web-platform-tests/css/css-writing-modes/support/tcu-font \
		--ignore web-platform-tests/fonts/adobe-fonts/CSSFWOrientationTest \
		--ignore web-platform-tests/fonts/adobe-fonts/CSSHWOrientationTest \
		--workers 4

.PHONY: tests
.PHONY: tests-draw-selected tests-name-selected tests-sign-selected
.PHONY: tests-scan tests-sign
