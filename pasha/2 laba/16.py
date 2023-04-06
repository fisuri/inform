import random


class FormulaCalculator:
    def __init__(self, a0, b0, b1):
        self.a0 = a0
        self.b0 = b0
        self.b1 = b1
        self.y_prev = 0
        self.x_prev = 0

    def calculate(self, x):
        y = self.b1 * self.x_prev + self.b0 * x - self.a0 * self.y_prev
        self.x_prev = x
        self.y_prev = y
        return y


calculator = FormulaCalculator(1, 2, 3)
x = float(input("Введите значение переменной x: "))
y = calculator.calculate(x)
print("Значение выходной переменной y:", y)
