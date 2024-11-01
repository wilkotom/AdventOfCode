
def get_seat_id(seat: str) -> int:
    return int(seat.replace('F',"0").replace("B","1").replace("R","1").replace("L","0"),2)

seat_ids = [get_seat_id(seat) for seat in open("input.txt")]

print(f"Part 1: {max(seat_ids)}")

print(f"Part 2: {sum(range(min(seat_ids), max(seat_ids) +1)) - sum(seat_ids)}")

