import numpy as np

n = int(input())
# a = [list(map(int, input().split())) for _ in range(n)]
a = [input() for _ in range(n)]
keep = a[0][n-1]

a[0] = (a[1][0] + a[0][0:n-1])

for k in range(n-2):
    i = k+1
    l = keep
    keep = a[i][n-1]
    a[i] = (a[i+1][0] + a[i][1:n-1] + l)

a[n-1] = (a[n-1][1:n] + keep)

for i in a:
    print(i)


# print(a)
# one = a[0][0]

# for i in range(n-1):
#     a[0][i] = a[0][i+1]

# for i in range(n-1):
#     a[i][n-1] = a[i+1][n-1]

# for i in range(n-1):
#     a[n-1][n-1-i] = a[n-1][n-2-i]

# for i in range(n-2):
#     a[n-1-i][0] = a[n-2-i][0]
# a[2][0] = one

# print(a)