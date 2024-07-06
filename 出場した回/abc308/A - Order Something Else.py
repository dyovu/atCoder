n, p, q = [int(x) for x in input().split()]
d = [int(x) for x in input().split()]
min_cost = p

for i in d:
    if q+i<min_cost:
        min_cost = q+i

print(min_cost)

