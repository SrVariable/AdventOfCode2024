lines = (''.join(open('input.txt', 'r'))).split('\n')
lines.pop(len(lines) - 1)

rules = {}
sequences = []
for line in lines:
    if '|' in line:
        x, y = map(int, line.split('|'))
        if x not in rules:
            rules[x] = []
        rules[x].append(y)
    elif ',' in line:
        sequences.append(list(map(int, line.split(','))))

def is_valid_sequence(sequence):
    for i in range(len(sequence)):
        for j in range(i + 1, len(sequence)):
            if sequence[j] in rules.keys() and sequence[i] in rules[sequence[j]]:
                return False
    return True

def part_one():
    total_sum = 0
    for sequence in sequences:
        if is_valid_sequence(sequence):
            total_sum += sequence[len(sequence) // 2]
    print(total_sum)

def get_wrong_indexes(sequence):
    for i in range(len(sequence)):
        for j in range(i + 1, len(sequence)):
            if sequence[j] in rules.keys() and sequence[i] in rules[sequence[j]]:
                return [i, j]


def part_two():
    total_sum = 0
    for sequence in sequences:
        is_valid = is_valid_sequence(sequence)
        if not is_valid:
            while not is_valid:
                i, j = get_wrong_indexes(sequence)
                sequence[i], sequence[j] = [sequence[j], sequence[i]]
                is_valid = is_valid_sequence(sequence)
            total_sum += sequence[len(sequence) // 2]
    print(total_sum)

part_one()
part_two()
