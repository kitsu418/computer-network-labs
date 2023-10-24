#!/bin/sh
cd build && make
for i in $(seq 10); do
    echo "Test $i:"
    ./gbn ../input.txt ../output.txt >../gbn.log
    echo "Comparing GBN result..."
    diff ../input.txt ../output.txt
    ./sr ../input.txt ../output.txt >../sr.log
    echo "Comparing SR result..."
    diff ../input.txt ../output.txt
    ./tcp ../input.txt ../output.txt >../tcp.log
    echo "Comparing TCP result..."
    diff ../input.txt ../output.txt
done
cd ..
