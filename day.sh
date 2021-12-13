#!/bin/bash

if [ "$1" == "" ];
then
    echo "Please specify a day."
fi

mkdir "day-$1"
cd "day-$1"
touch main.rs
touch sample.txt
touch input.txt
cargo init --vcs none

exit 0