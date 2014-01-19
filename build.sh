#!/usr/bin/env bash

./test.sh

if [ $? -ne 0 ]
then
    echo "Tests failed"
    exit 1
fi

python apply_snippets.py > applied.md 

if [ $? -ne 0 ]
then
    echo "Merging failed"
    rm applied.md
    exit 1
fi

landslide config.cfg -i --relative --copy-theme

if [ $? -ne 0 ]
then
    echo "Landslide failed"
    rm applied.md
    exit 1
fi

rm applied.md
