def is_partial_vertically_symmetric(matrix):
    rows = len(matrix)
    if rows == 0:
        return True, 0  # An empty matrix is symmetric

    cols = len(matrix[0])
    for start in range(cols):  # Iterate through all potential starting points
        for mid in range(start+1, cols):  # Iterate through potential symmetry lines
            is_symmetric = True
            for i in range(rows):
                for j in range(start, mid):
                    mirror_j = cols - 1 - (mid - j)  # Calculate the mirrored index
                    if mirror_j >= cols or matrix[i][j] != matrix[i][mirror_j]:
                        is_symmetric = False
                        break
                if not is_symmetric:
                    break

            if is_symmetric:
                return True, mid  # Found a symmetric slice

    return False, -1  # No symmetric slice found

if __name__=="__main__":
    part1Result = 0
    sky = None
    with open("2023/day13/input.txt") as f:
        spaceInput = [list(line.strip()) for line in f.readlines()]

    space = [[point for point in list(spaceInput[i])] for i in range(len(spaceInput))]
    space = [0, 1, 2, 3, 4, 5, 5, 4, 3, 2, 1],
    print(is_partial_vertically_symmetric(space))