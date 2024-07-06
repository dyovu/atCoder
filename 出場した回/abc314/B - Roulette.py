n = int(input())
x = []
y = []
for _ in range(n):
    x.append(int(input()))
    y.append(list(map(int, input().split())))

z = int(input())

hit=[]
max = 40

# print("")
# print(x)
# print(y)

for i in range(n):
    if z in y[i]:
        if len(y[i]) < max:
            hit = []
            hit.append(i+1)
            max = len(y[i])
        elif len(y[i]) == max:
            hit.append(i+1)

print(len(hit))
print(*hit)
