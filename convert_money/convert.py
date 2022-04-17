def convert(num):
    """convert number to the representation of Chinese money"""
    splited = _split(num)

    primary_unit = ["", "万", "亿",  "兆"]
    prefix_unit = ["", "拾", "佰", "仟"]
    bits = {
        0: "零", 1: "壹", 2: "贰", 3: "叁", 4: "肆",
        5: "伍", 6: "陆", 7: "柒", 8: "捌", 9: "玖"
    }

    #     ...    亿 亿 亿  亿  万 万 万 万
    #     ...    千 百 十     千 百 十     千 百 十
    # i = ... 11 10 9  8  7  6  5  4  3  2  1 0
    result = []
    for (i, p) in enumerate(splited[2:]):
        if p == 0:
            result.append(f"{bits[0]}")
        else:
            primary = primary_unit[i // 4]
            prefix = prefix_unit[i % 4]
            result.append(f"{bits[p]}{prefix}{primary}")

    result.reverse()
    result.append("元")

    # the splited[1] is x of .xy, splited[0] is y of .xy
    if splited[1] != 0:
        result.append(f"{bits[splited[1]]}角")
    if splited[0] != 0:
        result.append(f"{bits[splited[0]]}分")

    return "".join(result)


def _split(num):
    """split the num to a list of every bits of it"""
    # xxxx.xx => xxxxxx
    num = num * 100

    result = []
    for i in range(16):
        tmp = num // 10 ** i
        if tmp == 0:
            return result
        result.append(tmp % 10)

    return result


def main():
    a = 10234506007890.02
    print(convert(a))


if __name__ == "__main__":
    main()