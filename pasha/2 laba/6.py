from math import *
from random import randint

a = [randint(1, 100) for i in range(10)]
several = 0

for i in a:
    several += i

print(a)
print(several/len(a))
