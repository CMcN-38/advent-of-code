from utils.api import get_input
import math
import re

input_str = get_input(14)

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

run_str = input_str

# WRITE YOUR SOLUTION HERE

def parse_input(string):
    transposed = '\n'.join(''.join(row) for row in zip(*string.split('\n')))
    new_string = []
    #tilt
    for line in transposed.splitlines():
        no_swaps = False
        while no_swaps == False:
            no_swaps = True
            for i in range(0, len(line)):
                if i == 0:
                    continue
                else:
                    if line[i] == "O" and line[i - 1] == ".":
                        line = list(line)
                        line[i - 1] = "O"
                        line[i] = "."
                        line = "".join(line)
                        no_swaps = False
                    else:
                        continue
        new_string.append(line)
    new_string = "\n".join(new_string)
    new_string = "\n".join(''.join(row) for row in zip(*new_string.split('\n')))
    return new_string

def count_load(string):
    rows = len(string.split("\n"))
    total = 0
    for line in string.split("\n"):
        total += line.count("O") * rows
        rows -= 1

    return total

parsed_string = parse_input(run_str)

print(f'Part 1 Answer: {count_load(parsed_string)}')

# Part 2 

def tilt_left(string):
    #tilt
    new_string = []
    for line in string.splitlines():
        no_swaps = False
        while no_swaps == False:
            no_swaps = True
            for i in range(0, len(line)):
                if i == 0:
                    continue
                else:
                    if line[i] == "O" and line[i - 1] == ".":
                        line = list(line)
                        line[i - 1] = "O"
                        line[i] = "."
                        line = "".join(line)
                        no_swaps = False
                    else:
                        continue
        new_string.append(line)
    new_string = "\n".join(new_string)
    return new_string

def tilt_right(string):
    #tilt
    new_string = []
    for line in string.splitlines():
        no_swaps = False
        while no_swaps == False:
            no_swaps = True
            for i in range(0, len(line)):
                if i == len(line) - 1:
                    continue
                else:
                    if line[i] == "O" and line[i + 1] == ".":
                        line = list(line)
                        line[i + 1] = "O"
                        line[i] = "."
                        line = "".join(line)
                        no_swaps = False
                    else:
                        continue
        new_string.append(line)
    new_string = "\n".join(new_string)
    return new_string


def parse_input(string):
    transposed = '\n'.join(''.join(row) for row in zip(*string.split('\n')))
    new_string = tilt_left(transposed)

    new_string = "\n".join(''.join(row) for row in zip(*new_string.split('\n')))

    new_string = tilt_left(new_string)

    transposed = '\n'.join(''.join(row) for row in zip(*new_string.split('\n')))
    new_string = tilt_right(transposed)

    new_string = "\n".join(''.join(row) for row in zip(*new_string.split('\n')))

    new_string = tilt_right(new_string)

    return new_string

def count_load(string):
    rows = len(string.split("\n"))
    total = 0
    for line in string.split("\n"):
        total += line.count("O") * rows
        rows -= 1

    return total

parsed_string = run_str

for i in range(0, 1000):
    parsed_string = parse_input(parsed_string)

print(f'Part 2 Answer: {count_load(parsed_string)}')

