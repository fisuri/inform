from math import *

num = int(input('Введите число : '))
a = int(input('Введите число разрядов : '))


def b(num, a):
    return str(num)[:a:]


print(b(num, a))
