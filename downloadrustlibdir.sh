#!/bin/bash

URL="http://static.rust-lang.org/dist/rust-nightly-i686-unknown-linux-gnu.tar.gz"
TARPATH=rust-nightly-i686-unknown-linux-gnu/lib/rustlib/i686-unknown-linux-gnu/lib/
COMPONENTS=5
FOLDER=rustlibdir

rm -rf $FOLDER
mkdir $FOLDER
cd $FOLDER

curl $URL | tar -xz --strip-components $COMPONENTS $TARPATH
