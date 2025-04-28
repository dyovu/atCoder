n = int(input())
array1 = list(map(int, input().split()))
num = set()
set_length = 0
array2 = []

print(array1)
for i in array1:
  num.add(i)
  if set_length == len(num):
    array2.append(i)
  else:
    set_length=len(num)

print(array2)
num = set()
set_length = 0


for i in array2:
  num.add(i)
  if set_length != len(num):
    set_length=len(num)
    print(i)
  else:
    set_length=len(num)
