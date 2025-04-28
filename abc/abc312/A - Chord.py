s = input()
lit =["ACE","BDF","CEG","DFA","EGB","FAC","GBD"]
ans ="No"

for i in lit:
    if i == s:
        ans = "Yes"

print(ans)