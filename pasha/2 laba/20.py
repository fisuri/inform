def has_repeating_chars(word):
    return len(set(word)) < len(word)


print(has_repeating_chars(input()))
