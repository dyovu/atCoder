n = int(input())
a =[list(map(int, input().split())) for _ in range(n)]
x, y = [list(i) for i in zip(*a)]

maximum = 0
satisfication = {}

for i in range(n):
    satisfication[i] = y[i]

satisfication = sorted(satisfication.items(), key=lambda x:x[1], reverse=True)
# print(satisfication)

for i in range(n-1):
    if x[satisfication[0][0]] == x[satisfication[i+1][0]]:
        z = satisfication[i+1][1]/2
    else:
        z = satisfication[i+1][1]
    # print(z)
    maximum = max(maximum, (satisfication[0][1] + z))

print(int(maximum))
