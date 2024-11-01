sled_valid = 0
toboggan_valid = 0
with open("input.txt") as f:
    for line in f.readlines():
        limits, letter, password = line.split()
        letter = letter[0]
        lower, upper = [int(x)-1 for x in limits.split('-')]
        if lower + 1 <= password.count(letter) <= upper + 1:
            sled_valid += 1
        if (password[lower] == letter and password[upper] != letter) or \
                (password[upper] == letter and password[lower] != letter):
            toboggan_valid += 1


print(f"Valid Sled Co Passwords:: {sled_valid}")
print(f"Valid Toboggan Co Passwords: {toboggan_valid}")