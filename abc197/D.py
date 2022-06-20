import math

n = int(input())
x0, y0 = list(map(int, input().split()))
xn2, yn2 = list(map(int, input().split()))

r = pow((x0 - xn2)**2 + (y0 - yn2)**2, 0.5)
center = (((x0 + xn2) / 2) , ((y0 + yn2) / 2))

theta = math.radians(360 / n)
x_a = (x0 - center[0])
y_b = (y0 - center[1])
X_a = x_a * math.cos(theta) - math.sin(theta) * y_b
Y_b = math.sin(theta) * x_a + math.cos(theta) * y_b
X = X_a + center[0]
Y = Y_b + center[1]
print(X, Y)
