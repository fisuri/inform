class TriangleChecker:
    def __init__(self):
        a = float(input("Введите длину первой стороны: "))
        b = float(input("Введите длину второй стороны: "))
        c = float(input("Введите длину третьей стороны: "))

        # Проверка введенных значений на положительность
        if a <= 0 or b <= 0 or c <= 0:
            raise ValueError("Длины сторон должны быть положительными числами")

        # Сохранение введенных значений в качестве атрибутов
        self.a = a
        self.b = b
        self.c = c

    def is_triangle(self):
        # Проверка на возможность построения треугольника
        if self.a + self.b > self.c and self.b + self.c > self.a and self.c + self.a > self.b:
            return "Ура, можно построить треугольник!"
        elif self.a <= 0 or self.b <= 0 or self.c <= 0:
          # Проверка на отрицательные значения
            return "С отрицательными числами ничего не выйдет!"
        else:
            return "Жаль, но из этого треугольник не сделать."


# Создание экземпляра объекта TriangleChecker
my_triangle = TriangleChecker()

# Вызов метода is_triangle() и вывод результата
print(my_triangle.is_triangle())
