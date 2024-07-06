N = int(input())

a = [int(x) for x in input().split()]
p = [int(x) for x in input().split()]

for i in range(100):
    for j in range(len(p)):
        a[p[j]-1] = a[p[j]-1] + a[j+1]
                    
if a[0] == 0:
    print(0)
if a[0] > 0:
    print("+")
if a[0] < 0:
    print("-")

