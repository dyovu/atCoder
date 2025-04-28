s = input()
ans = ""

for i in s:
    if i != "a" and i != "e" and i != "i" and i != "o" and i != "u":
        ans += i

print(ans)