#!/bin/bash

set -e

TARGET=i686-unknown-linux-gnu
TARPATH=rust-nightly-${TARGET}/rustc/lib/rustlib/${TARGET}/lib/
COMPONENTS=6
FOLDER=rustlibdir

RUST_TARGET=$(rustc --version --verbose | grep host | awk '{print $2}')
RUST_DATE=$(rustc --version --verbose | grep commit-date | awk '{print $2}')
URL="http://static.rust-lang.org/dist/${RUST_DATE}/rust-nightly-${TARGET}.tar.gz"

echo "Downloading nightly of ${RUST_DATE}"

if [ "$TARGET" = "$RUST_TARGET" ]
then
	echo "Host target is ok. No download needed."
	exit 0
fi

rm -rf $FOLDER
mkdir $FOLDER
cd $FOLDER

curl $URL | tar -xz --strip-components $COMPONENTS $TARPATH
