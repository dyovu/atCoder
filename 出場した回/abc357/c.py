
def sierpinski_carpet(level):
    if level == 0:
        return ["#"]
    
    prev_carpet = sierpinski_carpet(level - 1)
    size = len(prev_carpet)
    new_carpet = []
    
    for i in range(3):
        for row in prev_carpet:
            if i == 1:
                new_carpet.append(row + ' ' * size + row)
            else:
                new_carpet.append(row * 3)
    
    return new_carpet

def print_carpet(carpet):
    for row in carpet:
        print(row)

N = int(input())
carpet = sierpinski_carpet(N)
print_carpet(carpet)
