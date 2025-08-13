n,x = [int(x) for x in input().split()]
ans = [-1,-1,-1]

#for文2回
for i in range(n+1):
    for j in range(n+1-i):
        if x == i*10000 + j*5000 + (n-i-j)*1000:
            ans = [i,j,n-i-j]


print(*ans)




#for文3回

# for i in range(n+1):
#     for j in range(n+1-i):
#         for k in range(n+1-i-j):
#             if x == i*10000 + j*5000 + k*1000 and i+j+k == n:
#                 ans = [i,j,k]

# print(*ans)
