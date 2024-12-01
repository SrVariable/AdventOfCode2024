left = []
right = []

with open('input.txt', 'r') as f:
    line = f.readline()
    while line:
        left.append(int(line[0:5]))
        right.append(int(line[7:]))
        line = f.readline()

left.sort()
right.sort()

hashtable = {}

prev = 0
count = 0
for value in right:
    count += 1
    hashtable[value] = count
    if prev != value:
        if count > 1:
            hashtable[prev] += 1
        prev = value
        count = 0

total_sum = 0
for value in left:
    if value in hashtable:
        total_sum += value * int(hashtable[value])

print(total_sum)
