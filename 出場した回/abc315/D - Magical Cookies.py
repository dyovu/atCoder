import numpy as np
h, w = [int(x) for x in input().split()]
col = [input().split() for l in range(h)]

sum = 0

def line():
    line_bool = False
    for i in range(h):
        count = 0
        line_cookie ={}
        for j in col[i][0]:
            if j != ".":
                count +=1
                line_cookie[j] = 1
        if len(line_cookie)==1 and count>=2:
            col[i] = ['.'*w]
            line_bool = True
    return line_bool
            

def columun():
    colmun_bool = False
    for i in range(w):
        count = 0
        column_cookie ={}
        for j in range(h):
            if col[j][0][i] != ".":
                count +=1
                column_cookie[col[j][0][i]] = 1
        if len(column_cookie)==1 and count>=2:
            for k in range(h):
                col[k] = [col[k][0][0:i] + '.' + col[k][0][i+1:]]
            colmun_bool = True
    return colmun_bool
            
# print(col)

# while line_bool or colmun_bool:
#     line()
#     columun()
#     print(col)

while line() or columun():
    pass

for i in col:
    for j in i[0]:
        if j != ".":
            sum +=1

print(sum)
