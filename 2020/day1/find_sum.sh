#!/bin/bash

input="input"

while IFS= read -r line
do
    rem=$((2020-line))
    if grep ^${rem}$ $input
    then
        echo "Numers are: $line $rem"
        exit 0
    fi
done < "$input"
echo "No number found!"