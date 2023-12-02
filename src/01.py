from utils.api import get_input

input_str = get_input(1)

# WRITE YOUR SOLUTION HERE
total = 0

#For Part 1
#def extract_numbers(string):
#    numbers = [int(s) for s in string if s.isdigit()]
#    first = numbers[0]
#    last = numbers[-1]
#    combined = int(str(first) + str(last))
#    return combined if combined >= 10 else None

#for i in input_str.splitlines():
#    result = extract_numbers(i)
#    print(f"string: {i}, result: {result}")
#    total = total + result

#print(total)


#For Part 2
nums_words = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
reverse_words = [w[::-1] for w in nums_words]
word_lengths = [len(w) for w in nums_words]

def Forward(line, W, L):
    for i, c in enumerate(line):
        if c.isdigit():
            return c
        for j, w in enumerate(W):
            if line[i:i+L[j]] == w:
                return str(j+1)

    raise Exception('no digit in line:', line)

for row in input_str.splitlines():
        a = Forward(row, nums_words, word_lengths)
        b = Forward(row[::-1], reverse_words, word_lengths) # reverse line
        
        total += int(a+b)

print(total)




