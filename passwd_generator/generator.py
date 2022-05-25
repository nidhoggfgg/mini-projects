#!/usr/bin/env python3
import argparse
import hashlib
import random


def main() -> None:
    # deal with args and make a cli tool
    description = """
    A tool to generator passwd.
        use a unique string to generate safe passwd, and no need to remember it.

    for example:
        ./generator.py -un --target facebook --auth OTZ?

        every time use this instruction will generator the same passwd.
        so, just remember the `auth` and don't tell anybody.
    """
    parser = argparse.ArgumentParser(
        formatter_class=argparse.RawDescriptionHelpFormatter, description=description
    )
    parser.add_argument(
        "-d",
        "--digits",
        type=int,
        default=16,
        metavar="N",
        help="the number of digits of the passwd, default 16",
    )
    parser.add_argument(
        "-u",
        "--uppercase",
        action="store_true",
        default=False,
        help="add uppercase char to passwd, default not",
    )
    parser.add_argument(
        "-n",
        "--number",
        action="store_true",
        default=False,
        help="add number to passwd, default not",
    )
    # fmt: off
    parser.add_argument(
        "-s",
        "--symbols",
        nargs="+",
        default=[ ".", "@", "_", "-", ":", "!" ],
        help="the symbols used in passwd, default .@_-:!",
    )
    # fmt: on
    parser.add_argument(
        "--target",
        required=True,
        help="the target for generating passwd, like facebook or some thing you can remember",
    )
    parser.add_argument(
        "--auth",
        required=True,
        help="the unique string to indicate user, pls remember it and don't tell anybody!",
    )
    args = parser.parse_args()

    digits = args.digits
    uppercase = args.uppercase
    number = args.number
    symbols = args.symbols
    target = args.target
    auth = args.auth

    # put the auth, target, digits and salt to make a hash
    # changing the code here will cause the generated password to change!
    salt = "don't ​crack ​this!😱"
    hash_it = hashlib.md5()
    hash_it.update(auth.encode("utf-8"))
    hash_it.update(target.encode("utf-8"))
    hash_it.update(str(digits).encode("utf-8"))
    hash_it.update(salt.encode("utf-8"))
    seed = hash_it.hexdigest()

    # generate the passwd
    passwd = generator(digits, uppercase, number, symbols, seed)
    print(passwd)


def generator(
    digits: int, uppercase: bool, number: bool, symbols: list[str], seed: str
) -> str:
    """generate passwd

    Args:
        digits (int):
            the digits of passwd
        uppercase (bool):
            add the uppercase char to passwd or not
        number (bool):
            add the number char to passwd or not
        symbols (list[str]):
            the symbols used in passwd
        seed (str):
            the seed for random, make it return same passwd when giving same seed
    """
    # the digits of passwd can't under 6
    if digits < 6:
        print("the digits of passwd is too short!(<6)")
        raise ValueError

    # the all char used in passwd
    # ascii 97~122: a-z
    # ascii 48~57: 0-9
    # ascii 65~90: A-Z
    charset = [chr(c) for c in range(97, 123)]

    # the result of passwd
    result = []

    # init random a seed to make it return same passwd when using same seed every time
    random.seed(seed)

    # add number
    if number:
        chars = [str(chr(c)) for c in range(48, 58)]
        charset.extend(chars)
        # make there is at least one number in the passwd
        result.append(random.choice(chars))
        digits -= 1

    # add uppercase char
    if uppercase:
        chars = [chr(c) for c in range(65, 91)]
        charset.extend(chars)
        # make there is at least one uppercase char in the passwd
        result.append(random.choice(chars))
        digits -= 1

    # add symbols
    if not symbols:
        raise ValueError
    tmp = random.choice(symbols)
    charset.extend(symbols)
    # make there is at least one symbol in the passwd
    result.append(tmp)
    digits -= 1

    # random making the passwd
    for _ in range(digits):
        tmp = random.choice(charset)
        result.append(tmp)

    # disrupt the passwd
    random.shuffle(result)
    return "".join(result)


if __name__ == "__main__":
    # generator(16, True, True, [ ".", "@", "_", "-", ":", "!" ], "test_seed")
    main()
