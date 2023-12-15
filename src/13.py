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

def parse_input(string):
    output = []
    pattern = []
    for i, line in enumerate(string.splitlines()):
        if i == len(string.splitlines())-1:
            pattern.append(line)
            output.append(pattern)
        if line != '':
            pattern.append(line)
        else:
            output.append(pattern)
            pattern = []

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


print(total)
