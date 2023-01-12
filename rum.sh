#!/bin/sh

OUTPUT_F="data/output-big.csv"
for f in wuf20-71 wuf20-91 wuf50-218 wuf75-325;
#for f in wuf20-71R wuf20-91R wuf50-218R wuf75-325;
do
    for letter in M N Q R
    do
        echo "$f $letter"
        cat "data/$f/$f-$letter-opt.dat" | while IFS= read -r line
        do
            dataset=$(echo "$line" | awk '{ print $1 }')
            optimum=$(echo "$line" | awk '{ print $2 }')
            output=$(cargo run --quiet --release --\
                        -c 0.97 \
                        --tail-cut-length 2500 \
                        --tail-cut-method relative-deviation \
                        --min-temperature 1 \
                        --input "data/$f/$f-$letter/w$dataset.mwcnf")
            echo "$f $letter $dataset $optimum $output" >> $OUTPUT_F
        done
    done
done