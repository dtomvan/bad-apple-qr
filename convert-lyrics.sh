#!/bin/bash

LY=bad-apple-lyrics
UN=$LY-unsorted.txt
ST=$LY-sorted.txt
FT=$LY-filtered.txt

cargo build --release
./target/release/bad-apple-qr > $UN
cat $UN | sort > $ST
cat $ST | grep -E "^.*\.qoi:::[a-z ]+$" | grep -v 'Ð' > $FT
cat $FT | sort | awk -F":::" '{ print $2 }' | uniq > $LY.txt
