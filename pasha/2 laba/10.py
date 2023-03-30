import numpy as np
width = int(input())
height = int(input())
a = np.random.randint(-10, 10, size=(height, width))

print(a)
print("След массива: ", np.trace(a))
