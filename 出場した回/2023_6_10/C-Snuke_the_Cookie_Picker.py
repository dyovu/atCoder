N = list(map(int, input().split()))
L = [input().split() for l in range(N[0])]
p = 0
q = 0
r = 0
l = 0
u = 0
d = 0
t = 0

for i in range(N[0]):
    p = i+1
    for j in range(N[1]):
        q = j+1
        if i == 0:
            u = 0
        elif L[i-1][0][j] == ".":
            u = 0
        elif L[i-1][0][j] == "#":
            u = 1
        if i == N[0]-1:
            d = 0
        elif L[i+1][0][j] == ".":
            d = 0
        elif L[i+1][0][j] == "#":
            d = 1
        if j == N[1]-1:
            r = 0
        elif L[i][0][j+1] == ".":
            r = 0
        elif L[i][0][j+1] == "#":
            r = 1
        if j == 0:
            l = 0
        elif L[i][0][j-1] == ".":
            l = 0
        elif L[i][0][j-1] == "#":
            l = 1
        t = r+l+u+d
        if t >= 2 and L[i][0][j]==".":
            break
    if t >= 2 and L[i][0][j]==".":
        break
print(p,q)