n, k = [int(x) for x in input().split()]

l = [list(map(int, input().split())) for _ in range(n)]

print(l)
count = 0
sum =0

while(True):
    for i in l:
        if i[0] >= 0:
            sum += i[1]
            i[0] -= 1
    if sum < k:
        print(count)
    else:
        count += 1