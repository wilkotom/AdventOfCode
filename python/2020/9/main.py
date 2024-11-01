def get_pairs(nums: list[int]) -> list[int]:
    return [nums[0] + n for n in nums[1:]] + get_pairs(nums[1:]) if nums else []


def get_contiguous(nums: list[int], target: int) -> list[int]:
    total = sum(nums)
    left = 0
    right = len(nums) - 1
    while total != target:
        if total > target:
            total -= nums[right]
            right -=1
        else:
            total += nums[right+1] - nums[left]
            left += 1
            right += 1
    return nums[left:right]


def main(filename: str, preamble_size: int) -> None:
    numbers = [int(l) for l in open(filename)]
    solution = 0
    for i, val in enumerate(numbers[preamble_size:]):
        if val not in get_pairs(numbers[i:i+preamble_size]):
            solution = val
            break
    print(f"Part 1: {solution}")
    contiguous = get_contiguous(numbers[0:numbers.index(solution)], solution)
    print(f"Part 2: {min(contiguous) + max(contiguous)}")


if __name__ == "__main__":
    main("input.txt", 25)
