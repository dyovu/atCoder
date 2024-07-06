m = int(input())
day = [int(x) for x in input().split()]

half_sum = int((sum(day)+1)/2)
i = 0

while True:
    if half_sum > day[i]:
        half_sum -= day[i]
        i += 1
    else:
        print(i+1, half_sum)
        break

