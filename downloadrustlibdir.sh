#!/bin/bash

TARGET=i686-unknown-linux-gnu
URL="http://static.rust-lang.org/dist/rust-nightly-${TARGET}.tar.gz"
TARPATH=rust-nightly-${TARGET}/rustc/lib/rustlib/${TARGET}/lib/
COMPONENTS=6
FOLDER=rustlibdir

RUST_TARGET=$(rustc --version --verbose | grep host | awk '{print $2}')

if [ "$TARGET" = "$RUST_TARGET" ]
then
	echo "Host target is ok. No download needed."
	exit 0
fi

rm -rf $FOLDER
mkdir $FOLDER
cd $FOLDER

curl $URL | tar -xz --strip-components $COMPONENTS $TARPATH
