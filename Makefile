all:

configure:
	cargo install font --all-features

prepare: prepare-png
	mkdir -p assets/png
	qlmanage -t -s 224 -o asserts/png assets/svg

prepare-png: prepare-svg
	mkdir -p assets/png

prepare-svg:
	mkdir -p assets/svg
	font-sign -- \
		--input tests/fixtures \
		--output assets/svg \
		--characters anop \
		--ignore google-fonts/ofl/bungeecolor \
		--ignore google-fonts/ofl/bungeespice \
		--ignore google-fonts/ofl/gruppo \
		--ignore google-fonts/ofl/iceland \
		--ignore google-fonts/ofl/kaushanscript \
		--ignore google-fonts/ufl/ubuntu \
		--workers 4

.PHONY: all configure prepare prepare-png prepare-svg
