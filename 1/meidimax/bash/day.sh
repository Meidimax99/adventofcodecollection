#!/bin/bash
if [ -z ${1+x} ];
    then ARG=1
    else ARG=$1
fi
cat ../input.txt | sed -zr 's/(\n)([^\n])/\t\2/g' | awk '{for(i=1;i<=NF;i++){NUM=NUM?NUM+$i:$i};print(NUM);NUM=""}' | sort -n | tail -n $ARG | awk '{sum+=$0} END {print sum}' 
