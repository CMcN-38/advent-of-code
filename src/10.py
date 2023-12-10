from utils.api import get_input

input_str = get_input(10)

test_str = '''7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ'''

run_str = input_str
# WRITE YOUR SOLUTION HERE

def parse_to_grid(string):
    rows = string.strip().splitlines()
    grid = [list(row) for row in rows]
    
    return grid

def find_start(grid):
    for y in range(len(grid)):
        for x in range(len(grid[0])):
            if grid[y][x] == "S":
                return (y, x)
    return None

def get_loc_adj(grid, start_position):
    max_row = len(grid)
    max_col = len(grid[0]) - 1 if max_row > 0 else 0
    row, col = start_position
    adj_pos = [
        (row - 1, col), #up
        (row + 1, col), #down
        (row, col - 1), #left
        (row, col + 1), #right
    ]
    
    adj_val = []
    for rows, cols in adj_pos:
        if 0 <= rows < max_row and 0 <= cols <= max_col:
            adj_val.append(grid[rows][cols])
        else:
            adj_val.append("0")
    return max_row, max_col, row, col, adj_pos, adj_val

def new_move(direction, start_position, adjacent_pos, adjacent_val):
    if direction == "up":
        prev_pos = start_position
        new_pos = adjacent_pos[0]
        new_val = adjacent_val[0]
    elif direction == "down":
        prev_pos = start_position
        new_pos = adjacent_pos[1]
        new_val = adjacent_val[1]
    elif direction == "left":
        prev_pos = start_position
        new_pos = adjacent_pos[2]
        new_val = adjacent_val[2]
    elif direction == "right":
        prev_pos = start_position
        new_pos = adjacent_pos[3]
        new_val = adjacent_val[3]
    else:
        return None
    return prev_pos, new_pos, new_val

def move_from_start(grid, start_position):
    
    max_row, max_col, row, col, adjacent_pos, adjacent_val = get_loc_adj(grid, start_position)

    if adjacent_val[0] in ("|", "F", "7"):
        direction = "up"
    elif adjacent_val[1] in ("|", "L", "J"):
        direction = "down"
    elif adjacent_val[2] in ("-", "L", "F"):
        direction = "left"
    elif adjacent_val[3] in ("-", "7", "J"):
        direction = "right"
    else:
        return None

    prev_pos, new_pos, new_val = new_move(direction, start_position, adjacent_pos, adjacent_val)

    return prev_pos, new_pos, new_val

def nav_grid(grid, prev_pos, new_pos, new_val, prev_val):
    max_row, max_col, row, col, adj_pos, adj_val = get_loc_adj(grid, new_pos)
#    print(f'prev_pos: {prev_pos}, new_val: {new_val}')      
#    print(f'Debug:')
#    print(f'max_row: {max_row}, max_col: {max_col}, row: {row}, col: {col}, adj_pos: {adj_pos}, adj_val: {adj_val}')

    direction = None
    if new_val == "|":
        if prev_pos == adj_pos[0]:
            direction = "down"
        else:
            direction = "up"
    elif new_val == "-":
        if prev_pos == adj_pos[3]:
            direction = "left"
        else:
            direction = "right"
    elif new_val == "L":
        if prev_pos == adj_pos[0]:
            direction = "right"
        else:
            direction = "up"
    elif new_val == "F":
        if prev_pos == adj_pos[1]:
            direction = "right"
        else:
            direction = "down"
    elif new_val == "7":
        if prev_pos == adj_pos[1]:
            direction = "left"
        else:
            direction = "down"
    elif new_val == "J":
        if prev_pos == adj_pos[0]:
            direction = "left"
        else:
            direction = "up"
    else:
        return None

    prev_val = new_val
    prev_pos, new_pos, new_val = new_move(direction, new_pos, adj_pos, adj_val)
    return prev_pos, new_pos, new_val, prev_val



grid = parse_to_grid(run_str)

start = find_start(grid)
prev_pos, new_pos, new_val = move_from_start(grid, start)

steps = 1
prev_val = "S"
while True:
    prev_pos, new_pos, new_val, prev_val = nav_grid(grid, prev_pos, new_pos, new_val, prev_val)
    steps += 1
    if new_val == "S":
        break 

result = steps/2

print(f'Part 1 Answer: {result}')


#Part 2:

def count_chars(grid):
    count = 0
    def within_loop(x, y):
        return 0 <= x < len(grid) and 0 <= y < len(grid[0]) and visited[x][y] == 1

    grid = [list(row) for row in grid.splitlines()]
    rows, cols = len(grid), len(grid[0])

    visited = [[0 for _ in range(cols)] for _ in range(rows)]

    start_x, start_y = -1, -1
    for i in range(rows):
        for j in range(cols):
            if grid[i][j] == 'X':
                start_x, start_y = i, j
                break
    if start_x == -1 or start_y == -1:
        return 0

    stack = [(start_x, start_y)]
    enclosed_count = 0

    while stack:
        x, y = stack.pop()
        if within_loop(x, y):
            enclosed_count += 1

        visited[x][y] = 1

        moves = [(x-1, y), (x+1,y), (x,y-1), (x,y+1)]

        for new_x, new_y in moves:
            if within_loop(new_x, new_y) or (0 <= new_x < rows and 0 <= new_y < cols):
                enclosed_count += 1
            if 0 <= new_x < rows and 0 <= new_y < cols and grid[new_x][new_y] == 'X' and visited[new_x][new_y] == 0:
                stack.append((new_x, new_y))
    return enclosed_count


grid = parse_to_grid(run_str)

start = find_start(grid)
prev_pos, new_pos, new_val = move_from_start(grid, start)

prev_val = "S"
steps = 0
while True:
    prev_pos, new_pos, new_val, prev_val = nav_grid(grid, prev_pos, new_pos, new_val, prev_val)
    steps += 1
    row, col = prev_pos
    grid[row][col] = "X"
    if new_val == "S":
        break 

for y in range(len(grid)):
    for x in range(len(grid[0])):
        if grid[y][x] == "S":
            grid[y][x] = "X"

new_grid = '\n'.join([''.join(sublist) for sublist in grid])

result = count_chars(new_grid)
print(f'Part 2 Answer: {result}')
