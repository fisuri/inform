def spinWords(sentence):
    words = sentence.split()
    result = ""
    for word in words:
        if len(word) >= 5:
            result += word[::-1] + " "
        else:
            result += word + " "
    return result.strip()


input_sentence = input("Введите предложение: ")
new_sentence = spinWords(input_sentence)
print(new_sentence)

