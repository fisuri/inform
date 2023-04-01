def replace_letters_with_numbers(text):
    alphabet = "abcdefghijklmnopqrstuvwxyz"
    result = [str(alphabet.find(letter.lower()) + 1) for letter in text]
    return ' '.join(result)


text = input("Введите строку на латинице: ")
numbers = replace_letters_with_numbers(text)
print(numbers)
