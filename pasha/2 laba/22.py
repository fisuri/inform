def generate_route(x, y):
    route = ""
    if y < 0:
        route += "s" * abs(y)
    else:
        route += "n" * y
    if x < 0:
        route += "w" * abs(x)
    else:
        route += "e" * x
    return route


x = int(input("Введите координату X: "))
y = int(input("Введите координату Y: "))
route = generate_route(x, y)
print("Маршрут до точки ({}, {}): {}".format(x, y, route))
