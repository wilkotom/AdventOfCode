numbers = set()

for number in [int(x) for x in open("input.txt")]:
    if 2020 - number in numbers:
        print(number * (2020 - number))
    numbers.add(number)

nums = sorted(list(numbers))

for i, n in enumerate(nums[:-2]):
    left = i + 1
    right = len(nums) - 1
    while left < right:
        if nums[i] > 674:
            break
        total = nums[i] + nums[left] + nums[right]
        if total == 2020:
            print(nums[i]*nums[left]*nums[right])
            left += 1
            right -= 1
        elif total > 2020:
            right -= 1
        else:
            left += 1
