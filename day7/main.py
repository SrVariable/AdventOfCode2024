#lines = (''.join(open('example.txt', 'r'))).split('\n')
lines = (''.join(open('input.txt', 'r'))).split('\n')
lines.pop(len(lines) - 1)

def calculate_combinations(equations, i=1, current=None, results=None):
    if results is None:
        results = []
    if current is None:
        current = equations[0]
    if i == len(equations):
        results.append(current)
        return results
    calculate_combinations(equations, i + 1, current + equations[i], results)
    calculate_combinations(equations, i + 1, current * equations[i], results)
    calculate_combinations(equations, i + 1, int(str(current) + str(equations[i])), results)
    return results


def is_valid_combination(xs, key):
    results = calculate_combinations(xs)
    for result in results:
        if key == result:
            return True
    return False

equations = {}
total_sum = 0
for line in lines:
    result, nums = line.split(':')
    result = int(result)
    equations[result] = list(map(int, nums.strip().split(' ')))
    if is_valid_combination(equations[result], result):
        total_sum += result

print(total_sum)
