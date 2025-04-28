h,w,n = [int(x) for x in input().split()]

current_x = int(0)
current_y = int(0)
current_color ='w'
direction = ['u','r','d','l']
current_direction = 0

grid =[]

for i in range(h):
  grid.append('.'*w)


def move():
  global current_x, current_y
  if direction[current_direction] == 'u':
    current_y -= 1
    if current_y == -1:
      current_y = h-1
  elif direction[current_direction] == 'r':
    current_x += 1
    if current_x == w:
      current_x = 0
  if direction[current_direction] == 'd':
    current_y += 1
    if current_y == h:
      current_y = 0
  elif direction[current_direction] == 'l':
    current_x -= 1
    if current_x == -1:
      current_x = w-1

def check_color():
  global current_color
  if grid[current_y][current_x]=='.':
    current_color = 'w'
  elif grid[current_y][current_x]=='#':
    current_color = 'b'

def aaa():
  for i in range(h):
    print(grid[i])


for i in range(n):
  if current_color=='w':
    grid[current_y] = list(grid[current_y])
    grid[current_y][current_x] = '#'
    grid[current_y] = ''.join(grid[current_y])
    current_direction = (current_direction+1)%4
    move()
    check_color()

  elif current_color=='b':
    grid[current_y] = list(grid[current_y])
    grid[current_y][current_x] = '.'
    grid[current_y] = ''.join(grid[current_y])
    current_direction = (current_direction-1)%4
    move()
    check_color()

aaa()
