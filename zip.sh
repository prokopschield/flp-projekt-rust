#!/bin/sh

# stop on failure
set -eu

# remove old zip
rm -f xschie03.zip

# add files
zip -r xschie03.zip maps/ src/ tests/ Cargo.* *.md
