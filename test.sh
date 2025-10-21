#!/bin/bash
set -eux

uv --version
tmplot="uv run python3 -m tmplot"

cat data/sample.txt | $tmplot plot -f - -l "a" -o test1.png
cat data/sample.txt | $tmplot scatter -f - -l "a" -o test2.png
cat data/sample.txt | awk '{print $2}' | $tmplot hist -f - -l "a" -o test3.png
cat data/sample.xvg | $tmplot plot -f - -l "a" -o test1_2.png

$tmplot cat plot -f data/sample.txt data/sample.xvg -l "a" "b" -o test4.png
$tmplot cat scatter -f data/sample.txt data/sample.xvg -l "a" "b" -o test5.png
$tmplot cat hist -f data/sample.txt data/sample.xvg -l "a" "b" -o test6.png

cat data/sample2.txt | $tmplot twin plot -f - -l "a" "b" -o test7.png
cat data/sample2.txt | $tmplot twin scatter -f - -l "a" "b" -o test8.png
