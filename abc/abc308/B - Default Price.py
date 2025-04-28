
n ,m = map(int ,input().split())
c = [x for x in input().split()]
d = [x for x in input().split()]
p = [int(x) for x in input().split()]

sum = 0
set_d  =set(d)

for i in c:
    if i in set_d:
        sum += p[d.index(i) +1]
    else:
        sum += p[0]

print(sum)