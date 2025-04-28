n, m = [int(x) for x in input().split()]
a = [int(x) for x in input().split()]
b = [int(x)+1 for x in input().split()]

c = a+b

a = sorted(a)
b = sorted(b, reverse=True)
c = sorted(c)

# def func():
#     if c[n] 

print(a)
print(b)
print(c)