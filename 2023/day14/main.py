
from typing import List

def find_indexes(lst: List[str], target: str) -> List[int]:
    return [i for i, x in enumerate(lst) if x == target]

def shiftRocks(platform: List[List[str]]):
    start = 1

    while start <= len(platform) - 1:
        sc = start
        while sc - 1 >= 0:
            zeroIndices = find_indexes(platform[sc], "O")
            for i in zeroIndices:
                if platform[sc - 1][i] == ".":
                    platform[sc][i] = "."
                    platform[sc - 1][i] = "O"
            sc -= 1
        
        start += 1
    
    return platform

def calc_load(platform: List[List[str]]) -> int:
    load = 0
    for i, row in enumerate(platform):
        zero_count = row.count("O")
        row_load = zero_count * (len(platform) - i)
        load += row_load
    return load

if __name__=="__main__":

    with open("2023/day14/input.txt") as f:
        platform = [list(line.strip()) for line in f.readlines()]

    print('\n'.join(''.join(row) for row in platform))

    platform = shiftRocks(platform)

    print("Moved the rocks")

    print('\n'.join(''.join(row) for row in platform))

    print(calc_load(platform))

