from itertools import pairwise

def findNext(numbers):
    if all(num == 0 for num in numbers):
        return 0
    
    diff = [(num2 - num1) for num1, num2 in pairwise(numbers)]
    
    return numbers[-1] + findNext(diff)

if __name__ == '__main__':
    part1Result = 0
    part2Result = 0

    with open('2023/day9/input.txt', 'r') as file:
        lines = file.read().split('\n')

    report = [[int(num) for num in lines[i].split()] for i in range(len(lines))]
    
    for values in report:
        part1Result += findNext(values)
        part2Result += findNext(list(reversed(values)))
    
    print("Part 1: ", part1Result)
    print("Part 2: ", part2Result)