def check_dirs(lines, i, j, rows, cols):
    count = 0
    if i - 1 >= 0 and i + 1 < rows and j - 1 >= 0 and j + 1 < cols:
        # M.S
        # .A.
        # M.S

        # M.M
        # .A.
        # S.S
        if lines[i - 1][j - 1] == 'M' and lines[i + 1][j + 1] == 'S':
            if (lines[i + 1][j - 1] == 'M' and lines[i - 1][j + 1] == 'S'
                or lines[i - 1][j + 1] == 'M' and lines[i + 1][j - 1] == 'S'):
                count += 1
        # S.S
        # .A.
        # M.M

        # S.M
        # .A.
        # S.M
        if lines[i + 1][j + 1] == 'M' and lines[i - 1][j - 1] == 'S':
            if (lines[i + 1][j - 1] == 'M' and lines[i - 1][j + 1] == 'S'
                or lines[i - 1][j + 1] == 'M' and lines[i + 1][j - 1] == 'S'):
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
        if (lines[i][j] == 'A'):
            total_sum += check_dirs(lines, i, j, rows, cols)

print(total_sum)
