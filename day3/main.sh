#/bin/bash

totalSum=0
while IFS= read -r line; do
	matches=($(echo $line | grep -oE "mul\([0-9]*,[0-9]*\)"))
	for match in ${matches[@]}; do
		pairs=($(echo $match | grep -oE "[0-9]*"))
		totalSum=$((totalSum + ${pairs[0]} * ${pairs[1]}))
	done
done < input.txt
echo $totalSum
