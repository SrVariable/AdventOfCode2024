#/bin/bash

totalSum=0
isEnabled=1
while IFS= read -r line; do
	matches=($(echo $line | grep -oE "mul\([0-9]*,[0-9]*\)|do\(\)|don't\(\)"))
	for match in ${matches[@]}; do
		if [ $match == "do()" ]; then
			isEnabled=1
		elif [ $match == "don't()" ]; then
			isEnabled=0
		else
			if [ $isEnabled == 1 ]; then
				pairs=($(echo $match | grep -oE "[0-9]*"))
				totalSum=$((totalSum + ${pairs[0]} * ${pairs[1]}))
			fi
		fi
	done
done < input.txt
echo $totalSum
