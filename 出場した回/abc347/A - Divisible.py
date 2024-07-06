n, k = [int(x) for x in input().split()]
a = [int(x) for x in input().split()]

b =[]

for i in a:
  if i%k == 0:
    b.append(i/k)

print(*b)