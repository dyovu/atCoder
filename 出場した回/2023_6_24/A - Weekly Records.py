n = int(input())
l = list(map(int, input().split()))
a=0
#print(len(l))

for i in range(len(l)):
    a += l[i]
    if (i+1)%7 == 0:
        print(a)
        a = 0
