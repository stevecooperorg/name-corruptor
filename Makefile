all: FORCE
	echo "nothing to do"

watch-py: FORCE
	cd py && nodemon --ext .py -x "pytest || exit 1"

.PHONY: FORCE
FORCE: