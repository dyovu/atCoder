n = int(input())
height = [int(x) for x in input().split()]

min_cost = [0]*n
min_cost[0] = 0
min_cost[1] = abs(height[1]-height[0])

for i in range(2,n):
    min_cost[i] = min(abs(height[i-1]-height[i]) + min_cost[i-1],
                      abs(height[i-2]-height[i]) + min_cost[i-2])

print(min_cost[n-1])
