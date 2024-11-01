card_public_key = 14222596
door_public_key = 4057428

steps = 0
subject_number = 7
next_val = 1

while next_val != card_public_key:
    steps += 1
    next_val = (next_val * subject_number) % 20201227
card_steps = steps

print(f"Private Key: {pow(door_public_key,card_steps, 20201227)}")


