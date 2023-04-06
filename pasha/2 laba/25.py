class Phone:
    def __init__(self, number, model, weight):
        self.number = number
        self.model = model
        self.weight = weight

    def get_number(self):
        return self.number

    def call(self, other_phone):
        print("Телефон", self.get_number(),
              "звонит на", other_phone.get_number())
        other_phone.receive_call(self)

    def receive_call(self, caller_phone):
        return self.get_number() + ' Вам звонит ' + caller_phone.get_number()


if __name__ == '__main__':

    phone1 = Phone("555-1234", "Poco X3 Pro", 150)
    phone2 = Phone("555-5678", "Huawei P30 Lite", 200)
    phone3 = Phone("555-9101", "Samsung A21S", 100)

    phone1.call(phone2)
