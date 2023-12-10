from utils.api import get_input

input_str = get_input(9)

test_str = '''0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45'''

run_str = input_str
# WRITE YOUR SOLUTION HERE

def parse_input(string):
    output = []
    for line in string.splitlines():
        list = line.split(' ')
        list = [int(i) for i in list]
        output.append(list) 
    return output

def calc_hist(list):
    total = 0
    while any(list):
        total += list[-1]
        list = [list[i + 1] - list[i] for i in range(len(list) - 1)]
    return total

result = 0
for list in parse_input(run_str):
   result += calc_hist(list) 

print(f'Part 1 Answer: {result}')

# PART 2:

result = 0
for list in parse_input(run_str):
    list.reverse()
    
    result += calc_hist(list) 

print(f'Part 2 Answer: {result}')
