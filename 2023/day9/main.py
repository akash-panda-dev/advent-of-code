def findNext_Part1(numbers):
    if all(num == 0 for num in numbers):
        return 0
    
    diff = [(num1- num2) for num1, num2 in zip(numbers[1:], numbers[:-1])]

    return numbers[-1] + findNext_Part1(diff)

def findNext_Part2(numbers):
    if all(num == 0 for num in numbers):
        return 0
    
    diff = [(num1- num2) for num1, num2 in zip(numbers[1:], numbers[:-1])]

    return numbers[0] - findNext_Part2(diff)


if __name__ == '__main__':
    part1Result = 0
    part2Result = 0

    with open('2023/day9/input.txt', 'r') as file:
        lines = file.read().split('\n')

    report = [[int(num) for num in lines[i].split()] for i in range(len(lines))]
    
    for values in report:
        part1Result += findNext_Part1(values)
    
    print("Part 1: ", part1Result)

    for values in report:
        part2Result += findNext_Part2(values)

    print("Part 2: ", part2Result)