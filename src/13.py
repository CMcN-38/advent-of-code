from utils.api import get_input
import math
import re

input_str = get_input(13)

test_str = '''#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#'''

run_str = input_str

# WRITE YOUR SOLUTION HERE
# The following is my code, it works on the test input but not on the real input
def parse_input(string):
    output = []

    for pattern in string.split("\n\n"):
        output.append(pattern.splitlines())

    return(output)

def find_symm(pattern):
    width = len(pattern[0])
    height = len(pattern)

    for col in range(width):
        is_symmetric = True
        for row in range(height):
            if pattern[row][col] != pattern[row][width - col - 1]:
                is_symmetric = False
                break

    return is_symmetric

def get_columns(pattern):
    cols = 0
    new_pattern = pattern
    rev_pattern = [line[::-1] for line in pattern]
    symm = False
    rev_symm= False
    count = 0
    while len(new_pattern[0]) > 1 and (symm == False or rev_symm == False):
        symm = find_symm(new_pattern)
        rev_symm = find_symm(rev_pattern)
        
        new_pattern = [line[1:] for line in new_pattern]
        rev_pattern = [line[1:] for line in rev_pattern]
        
        if symm == True:
            cols += math.ceil(len(new_pattern[0])/2)
            cols += count
            break
        if rev_symm == True:
            cols += math.ceil(len(rev_pattern[0])/2)
            break

        count += 1
    return cols

def transpose_pattern(pattern):
    transposed_pattern = [''.join(row) for row in zip(*pattern)]
    return transposed_pattern

patterns = parse_input(run_str)

total = 0

for i, pattern in enumerate(patterns):
    cols_res = get_columns(pattern)
    vert_res = get_columns(transpose_pattern(pattern))
    total += vert_res * 100
  #  if cols_res == 0:
  #      vert_res = get_columns(transpose_pattern(pattern))
  #      total += vert_res * 100
    total += cols_res


print(f'Part 1 Answer: {total}')

# The following is using a guide from HyperNeutrino

def find_mirror(grid):
    for r in range(1, len(grid)):
        above = grid[:r][::-1]
        below = grid[r:]

        above = above[:len(below)]
        below = below[:len(above)]

        if above == below:
            return r

    return 0

total = 0

for pattern in parse_input(run_str):
    row = find_mirror(pattern)
    total += row * 100

    new_pattern = transpose_pattern(pattern)
    col = find_mirror(new_pattern)
    total += col

print(f'Part 1 Answer using guide: {total}')


# Part 2

def find_mirror(grid):
    for r in range(1, len(grid)):
        above = grid[:r][::-1]
        below = grid[r:]

        if sum(sum(0 if a == b else 1 for a, b in zip(x, y)) for x, y in zip(above, below)) == 1:
            return r

    return 0

total = 0

for pattern in parse_input(run_str):
    row = find_mirror(pattern)
    total += row * 100

    new_pattern = transpose_pattern(pattern)
    col = find_mirror(new_pattern)
    total += col

print(f'Part 2 Answer using guide: {total}')


