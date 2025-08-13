n,W = map(int, input().split())
value=[0]*n
weight=[0]*n

dp = [[0] * (W + 1) for _ in range(n + 1)]

for i in range(n):
    value[i],weight[i] = map(int, input().split())


for i in range(n):
    for w in range(W+1):
        if w >= weight[i]:
            dp[i+1][w] = max(dp[i][w-weight[i]]+value[i],dp[i][w])
        else:
            dp[i+1][w] = dp[i][w]

print(dp[n][W])