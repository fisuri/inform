def check_route(route):
    x = 0
    y = 0
    for step in route:
        if step == "n":
            y += 1
        elif step == "s":
            y -= 1
        elif step == "e":
            x += 1
        elif step == "w":
            x -= 1
        else:
            return "Неверно задан маршрут"
    if x == 0 and y == 0:
        return True
    else:
        return False


user_route = input("Введите свой маршрут: ").split()
print(user_route)
print(check_route(user_route))
