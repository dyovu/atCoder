import sys

n = int(input())
l = A = [input() for _ in range(n)]

#print(l)

for i in range(len(l)):
    for j in range(int(i)+1,n):
        p =l[i] + l[j]
        q = p[::-1]
        s = l[j] + l[i]
        t = s[::-1]
        if p == q or s == t:
            print("Yes")
            sys.exit()

print("No")

