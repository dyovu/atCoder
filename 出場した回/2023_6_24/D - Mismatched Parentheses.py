n = int(input())
s = input()
array = {}
del_index =[]
l = len(s)
m = 0


for i in range(len(s)):
    array[i] = s[i]

for i in range(len(s)):
    if s[i] == "(":
        l = i
    elif s[i] == ")":
        m = i
        if l < m:
            del_index.append([l,m])

print(del_index)


print(array)