import math

N, S, M, L = map(int, input().split())

min_sum = float('inf')
res = N
print(math.ceil(N / 6))

for i in range(math.ceil(N / 6) + 1):
    res = N - i * 6
    if res < 0:
        if i*S<min_sum:
            min_sum = i*S
        break
    for j in range(math.ceil(res / 8)+1):
        res = N - i * 6 - j * 8
        if res < 0:
            if i* + j+M <min_sum:
                min_sum = i* + j+M
            break
        k = math.ceil(res / 12)
        sum_cost = i * S + j * M + k * L
        if sum_cost < min_sum:
            min_sum = sum_cost

print(min_sum)

