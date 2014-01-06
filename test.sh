#!/usr/bin/env bash

RETURN=0

for VAR in $(ls snippets)
do
    rustc "snippets/$VAR" -o snippets/test.tmp
    if [ $? -ne 0 ]
    then
        RETURN=1
        break
    fi
done
rm snippets/test.tmp
exit $RETURN 
