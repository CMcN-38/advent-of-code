from utils.api import get_input
import re

real_str = get_input(6)
test_str = '''Time:      7  15   30
Distance:  9  40  200
'''

input_str = real_str

# WRITE YOUR SOLUTION HERE

def separate_input(string):
    times = re.findall(r'[0-9]+',string.splitlines()[0])
    distances = re.findall(r'[0-9]+', string.splitlines()[1])    
    return times, distances

def sim_race(time, distance):
    count = 0
    for i in range(1, int(time)):
        if i*(time-i) > distance:    
            count += 1
    return count


times, distances = separate_input(input_str)

total = 1
for index, i in enumerate(times):
   total = total * sim_race(int(times[index]), int(distances[index]))
 
print(f'Part 1 Answer = {total}')

### Part 2

conc_times = ''.join(map(str, times))
conc_distances = ''.join(map(str, distances))

result = sim_race(int(conc_times), int(conc_distances))
print(f'Part 2 Answer = {result}')
