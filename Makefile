all: FORCE
	echo "nothing to do"

watch-pt: FORCE
	cd py && nodemon --ext .py -x "pytest"

.PHONY: FORCE
FORCE: