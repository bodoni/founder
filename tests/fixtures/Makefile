all: google-fonts-Noto_Sans_JP

%.zip:
	curl -L https://fonts.google.com/download?family=$$(echo "$*" | sed "s/_/%20/g") -o $@

google-fonts-%: %.zip
	unzip $< -d $@

.PHONY: all
