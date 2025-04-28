a,b = [int(x) for x in input().split()]

if a%3 != 0 and b == a+1:
    print("Yes")
else:
    print("No")