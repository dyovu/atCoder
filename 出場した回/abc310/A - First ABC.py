n = int(input())
s = input()
abc ={"A":0,"B":0,"C":0}

for i in s:
    abc[i] += 1
    # print(abc)
    if abc["A"]!=0 and abc["B"]!=0 and abc["C"]!= 0:
        print(abc["A"]+abc["B"]+abc["C"])
        break
