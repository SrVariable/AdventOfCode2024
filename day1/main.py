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

total_sum = 0
for i in range(0, len(left)):
    total_sum += abs(left[i] - right[i])

print(total_sum)
