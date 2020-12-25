card_public_key = 5764801
door_public_key = 17807724

steps = 0
subject_number = 7
next_val = 1

while next_val != card_public_key:
    steps += 1
    next_val = (next_val * subject_number) % 20201227
card_steps = steps

next_val = 1
for i in range(card_steps):
    next_val = (next_val * door_public_key) % 20201227

print(f"Private Key: {next_val}")


