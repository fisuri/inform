def addRightDgit(found_num, add_num):
    found_num = found_num*10+add_num
    return found_num


k = int(input("k: "))
d1 = int(input("d1: "))
print(addRightDgit(k, d1))
d2 = int(input("d2: "))
print(addRightDgit(addRightDgit(k, d1), d2))
