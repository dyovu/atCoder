ha, wa = map(int, input().split())
la = [input() for _ in range(ha)]
black_a = []

hb, wb = map(int, input().split())
lb = [input() for _ in range(hb)]
black_b = []

hx, wx = map(int, input().split())
lx = [input() for _ in range(hx)]
black_x = []



for i in range(len(la)):
    for j in range(wa):
        if la[i][j] == "#":
            black_a.append([i,j])
if black_a[0][0] != 0:
    h = black_a[0][0]
    for i in black_a:
        i[0]
print(black_a)

for i in range(len(lb)):
    for j in range(wb):
        if lb[i][j] == "#":
            black_b.append([i,j])
print(black_b)

for i in range(len(lx)):
    for j in range(wx):
        if lx[i][j] == "#":
            black_x.append([i,j])
print(black_x)


