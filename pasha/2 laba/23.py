class Soda:
    def __init__(self):
        self.addition = input("Введите добавку к газировке: ")

    def show_my_drink(self):
        if self.addition:
            print(f"Газировка и {self.addition}")
        else:
            print("Обычная газировка")


my_soda = Soda()
my_soda.show_my_drink()
