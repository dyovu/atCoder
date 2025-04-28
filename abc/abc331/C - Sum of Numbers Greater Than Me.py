N = int(input())
numlist = [int(x) for x in input().split()]

numdic = {index: num for index, num in enumerate(numlist)}

print(numdic)

numlist_sorted = sorted(numlist)

numset = set(numlist)
num_et_sorted = sorted(numset)

print( numlist_sorted)

for i in range(len(numlist_sorted)):
    if [i] == [i+1]