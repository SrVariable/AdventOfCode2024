# This could be simplified but I'll do it later
def check_dirs(lines, i, j, rows, cols):
    count = 0
    word = 'MAS'
    # Bottom right
    if i + 3 < rows and j + 3 < cols:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i + k][j + k]
        if temp == 'XMAS':
            count += 1
    # Bottom left
    if i + 3 < rows and j - 3 >= 0:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i + k][j - k]
        if temp == 'XMAS':
            count += 1
    # Top right
    if i - 3 >= 0 and j + 3 < cols:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i - k][j + k]
        if temp == 'XMAS':
            count += 1
    # Top left
    if i - 3 >= 0 and j - 3 >= 0:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i - k][j - k]
        if temp == 'XMAS':
            count += 1
    # Top
    if i - 3 >= 0:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i - k][j]
        if temp == 'XMAS':
            count += 1
    # Bottom
    if i + 3 < rows:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i + k][j]
        if temp == 'XMAS':
            count += 1
    # Left
    if j - 3 >= 0:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i][j - k]
        if temp == 'XMAS':
            count += 1
    # Right
    if j + 3 < rows:
        temp = 'X'
        for k in range(1, 4):
            temp += lines[i][j + k]
        if temp == 'XMAS':
            count += 1
    return count

lines = ''.join(open('input.txt', 'r'))
lines = lines.split('\n')
lines.pop(len(lines) - 1)

total_sum = 0
rows = len(lines)
cols = len(lines[0])
for i in range(rows):
    for j in range(cols):
        if (lines[i][j] == 'X'):
            total_sum += check_dirs(lines, i, j, rows, cols)

print(total_sum)
