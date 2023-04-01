def formula(x, a0, b0, b1, y_prev=0):
    y = b1*x + b0*x*y_prev - a0*y_prev
    return y


a0 = 0.5
b0 = 0.3
b1 = 0.2

x1 = 1
y1 = formula(x1, a0, b0, b1)
print(y1)

x2 = 2
y2 = formula(x2, a0, b0, b1, y1)
print(y2)

x3 = 3
y3 = formula(x3, a0, b0, b1, y2)
print(y3)
