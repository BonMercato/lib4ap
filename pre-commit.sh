#!/bin/sh

set -eo pipefail

if [ src/lib.rs -nt README.md ] || [ ! -f README.md ]
then
	cargo readme > README.md
	git add -A
fi
