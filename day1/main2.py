left = []
right = []

with open('input.txt', 'r') as f:
    line = f.readline()
    while line:
        left.append(int(line[0:5]))
        right.append(int(line[7:]))
        line = f.readline()

total_sum = 0
for value in left:
    total_sum += value * right.count(value);

print(total_sum)
