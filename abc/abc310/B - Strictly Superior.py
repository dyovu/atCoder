n, m = [int(x) for x in input().split()]
goods = [list(map(int, input().split())) for l in range(n)]

ans = "No"
values = []
functions = []
goods = sorted(goods)
for i in range(len(goods)):
    values += [goods[i][0]]
    functions += [goods[i][2:]]

# print(values)
# print(functions)
# print(len(functions)-1)

for i in range(len(functions)-1):
    for j in range(len(functions)-i-1):
        func_1 = sorted(functions[i])
        func_2 = sorted(functions[j+i+1])
        if len(func_1) >= len(func_2):
            for k in range(len(func_2)):
                if func_1[k] != func_2[k]:
                    break
                elif k+1 == len(func_2) and (values[i]<values[j+i+1] or len(func_1) > len(func_2)):
                    ans = "Yes"


print(ans)


