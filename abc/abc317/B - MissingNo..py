n = int(input())
num = [int(x) for x in input().split()] 

num_sorted = sorted(num)
ans = num_sorted[0]

for i in num_sorted:
    if i != ans:
        print(ans)
    else:
        ans += 1