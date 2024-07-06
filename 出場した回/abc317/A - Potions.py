n, m, x = [int(x) for x in input().split()]
p = [int(x) for x in input().split()]

for i in range(n):
    if x-m <= p[i]:
        print(i+1)
        break
