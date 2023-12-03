from utils.api import get_input
import re

input_str = get_input(3)

# WRITE YOUR SOLUTION HERE
## Got close by myself but no cigar, so ended up using code by RedPixel to get running. Learnt a lot but this day was very difficult for me.

def check_borders(string):
    sum = 0
    for y, line in enumerate(string):
        x = 0
        while x < len(line):
            numlen = 0
            if string[y][x].isdigit():
                while x < len(line) and string[y][x].isdigit():
                    numlen += 1
                    x += 1
                num = int(line[x - numlen : x])
                if any(
                    column not in "1234567890."
                    for i in range(max(0, y - 1), min(len(string), y + 2))
                    for column in string[i][max(0, x - numlen - 1) : min(len(line), x + 1)]
                ):
                    sum += num
            else:
                x += 1
    return sum

result = check_borders(input_str.splitlines())
print(f'Part 1: {result}')

## Part 2:

def find_num(line, x):
    a = b = x

    while a >= 0 and line[a].isdigit():
        a -= 1
    while b < len(line) and line[b].isdigit():
        b += 1
    return int(line[a + 1: b])

def check_gear(string):
    sum = 0

    for y, line in enumerate(string):
        for x, column in enumerate(line):
            if column == "*":
                a = list(
                    {
                        find_num(string[i], j)
                        for i in range(max(0, y - 1), min(len(string) + 1, y + 2))
                        for j in range(max(0, x - 1), min(len(line) + 1, x +2))
                        if string[i][j].isdigit()
                     }
                 )
                if len(a) == 2:
                    sum += a[0] * a[1]
    return sum

result = check_gear(input_str.splitlines())

print(f'Part 2: {result}')



