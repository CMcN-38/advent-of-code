from utils.api import get_input
import math
import re

#input_str = get_input(14)

test_str = '''O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....'''

run_str = test_str

# WRITE YOUR SOLUTION HERE

def parse_input(string):
    transposed = '\n'.join(''.join(row) for row in zip(*string.split('\n')))
    
    #tilt
    for line in transposed.splitlines():
        print(f'\n')
        print(line)
        line_list = list(line)

        zeros_indices = [i for i, char in enumerate(line_list) if char == 'O']

        for i in range(len(line_list)):
            if line_list[i] == 'O' and i > zeros_indices[0]:
                line_list[zeros_indices[0]], line_list[i] = line_list[i], line_list[zeros_indices[0]]
                zeros_indices[0] += 1
        result = ''.join(line_list)
        print(result)


parse_input(run_str)

