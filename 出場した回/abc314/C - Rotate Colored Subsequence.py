n, m = [int(x) for x in input().split()]
s = input()
col = [int(x) for x in input().split()]

replace ={}
origin_seq = {}
new_seq = {}
ans = ""

for i in range(m):
    replace[i+1] = []
    origin_seq[i+1] = []

for i in range(n):
    replace[col[i]] += s[i]
    origin_seq[col[i]] += [i]

# print(origin_seq)
# print(replace)

for i in replace:
    k = replace[i]
    last = k[len(k)-1]

    replace[i] = [k[len(k)-1]] + k[0:len(k)-1]
    # print(replace[i])


for i in replace:
    for j in range(len(replace[i])):
        new_seq[origin_seq[i][j]] = replace[i][j]
    # print(new_seq)

dic2 = new_seq = sorted(new_seq.items())

for x in dic2:
    ans += x[1]

print(ans)
