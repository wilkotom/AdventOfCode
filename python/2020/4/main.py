import re

required_fields = {"byr": "(19[2-9][0-9]|200[012])",
                   "iyr": "20([1][0-9]|20)",
                   "eyr": "20(2[0-9]|30)",
                   "hgt": "(1([5-8][0-9]|9[0-3])cm|(59|6[0-9]|7[0-6])in)",
                   "hcl": "#[0-9a-f]{6}",
                   "ecl": "(amb|b(lu|rn)|gr[yn]|hzl|oth)",
                   "pid": "[0-9]{9}"}


def validate_passport(passport: dict) -> bool:
    if 'cid' in passport:
        del passport['cid']
    if sorted(passport) != sorted(required_fields):
        return False
    return None not in [re.fullmatch(required_fields[f], passport[f]) for f in required_fields]

# Golfing this somewhat, because, why not?
def main() -> None:
    valid = [validate_passport(passport) for passport in
             [dict(x.split(':') for x in record.split()) for record in
              [d for d in open("input.txt").read().split('\n\n')]]].count(True)
    print(f"{valid} valid passports")


if __name__ == '__main__':
    main()