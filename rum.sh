#!/bin/sh


for f in \
 data/wuf20-91R/wuf20-91R-R/wuf20-0118.mwcnf \
 data/wuf50-218R/wuf50-218R-Q/wuf50-0311.mwcnf \
 data/wuf75-325/wuf75-325-M/wuf75-078.mwcnf
do
    cargo run --release --\
                -c 0.97 \
                --tail-cut-length 2500 \
                --tail-cut-method relative-deviation \
                --min-temperature 1 \
                --input $f

done