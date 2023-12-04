from utils.api import get_input
import re

input_str = get_input(4)
 
# WRITE YOUR SOLUTION HERE

test_str = '''Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11'''

def parse_win(string):
    #winning_nums = list()
    winning_nums.append(re.findall(r'Card[\s]+[0-9]+:[0-9\s]+', string))
    return winning_nums

def parse_our(string):
    our_nums.append(re.findall(r'\|[0-9\s]+', string))
    return our_nums

def sep_nums(list, num):
    output = []
    for sublist in list:
        for item in sublist:
            processed_item = item.split()
        output.append(processed_item[num:])
    return output

our_nums = list()
winning_nums = list()
for i in input_str.splitlines():
    winning_nums = parse_win(i)
    our_nums = parse_our(i)

winning_nums = sep_nums(winning_nums, 2)
our_nums = sep_nums(our_nums, 1)


total = 0
for s, sublist in enumerate(our_nums):
    count = 0
    for i, item in enumerate(sublist):
       if item in winning_nums[s]:
           count += 1
    if count > 0:
        result = 2 ** (count - 1)
    else:
        result = 0    
    total += result

print(f'The answer for Part 1 is {total}')

## Part 2:
def increment_card_count(list, start, end, value):
    for index in range(start, end + 1):
        list[index] += value
    return list

l = 203

our_nums = list()
winning_nums = list()
for i in input_str.splitlines():
    winning_nums = parse_win(i)
    our_nums = parse_our(i)

winning_nums = sep_nums(winning_nums, 2)
our_nums = sep_nums(our_nums, 1)

#card_count = [1 for _ in range(6)]
card_count = [1] * l

total = 0
for s, sublist in enumerate(our_nums):
    counter = 0
    i = 0
    if s > (l - 1):
        break
    while counter < card_count[s]:
        count = 0
        for i, item in enumerate(sublist):
            if item in winning_nums[s]:
                count += 1
        if (s + count) > (l - 1):
            card_count = increment_card_count(card_count, s+1,l - 1, 1)    
        else:
            card_count = increment_card_count(card_count,s+1,s+count,1)
        counter += 1

total = sum(card_count)



print(f'The answer for Part 2 is {total}')


