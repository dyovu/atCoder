n, d = [int(x) for x in input().split()]
s = [input() for _ in range(n)]

count =0
num_o = {}
streek = [0]*(d+1)
# print(streek)
l = 0

for i in range(d):
    num_o[i] = 0

for i in range(d):
    for j in s:
        if j[i] == "o":
            num_o[i] += 1
    # print(num_o)
    if num_o[i] != n:
        streek[i+1] = 0
    else:
        if streek[i] != 0:
            streek[i+1] = streek[i] + 1
        else:
            streek[i+1] = 1


print(max(streek))