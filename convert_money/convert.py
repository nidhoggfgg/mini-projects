#!/usr/bin/env python
import math
import sys
from typing import Callable, Generator, Optional


def convert(num: float) -> Optional[str]:
    """转换数字为规范的书写格式"""
    splited = _split(num)
    if splited is None:
        return None
    integer, (frac0, frac1) = splited

    # 超过 9999_9999_9999_9999
    if len(integer) > 16:
        return None

    chunks = []
    for chunk in _cut(integer):
        chunks.append(chunk)

    result = ""
    suffix = ["仟", "佰", "拾", ""]
    transer = ["零", "壹", "贰", "叁", "肆", "伍", "陆", "柒", "捌", "玖"]
    primarys = ["万亿", "亿", "万", ""]

    # 待填入 0 标志
    is_zero = False
    for i, chunk in enumerate(chunks[::-1]):
        index = 4 - len(chunks) + i
        tmp, (start_zero, end_zero) = _convert_chunk(
            chunk, suffix, transer, primarys[index]
        )

        # 稍后再填入 0，因为可能无需填 0
        if not tmp:
            is_zero = True
            continue

        if i > 0 and (is_zero or start_zero):
            result += "零"

        result += tmp

        # 尾部含 0 即待填入 0
        is_zero = end_zero

    full_zero = integer[-1] == 0 and len(integer) == 1
    if not full_zero:
        result += "圆"

    if not full_zero and frac0 == 0 and frac1 == 0:
        result += "整"
        return result

    if frac0 != 0:
        result += f"{transer[frac0]}角"

    if frac1 != 0:
        result += f"{transer[frac1]}分"

    return result


def _split(num: float) -> Optional[tuple[list[int], tuple[int, int]]]:
    """将数字划分为小数和整数部分，保留两位小数，不足补 0"""
    if not math.isfinite(num) or num < 0:
        return None

    tmp = f"{num}"
    snum = tmp.split(".")

    _integer = list(snum[0])
    integer = [int(x) for x in _integer]
    if len(snum) == 2:
        fracs = snum[1]
        if len(fracs) == 2:
            return integer, (int(fracs[0]), int(fracs[1]))

        return integer, (int(fracs[0]), 0)
    return integer, (0, 0)


def _convert_chunk(
    chunk: list[int], suffix: list[str], transer: list[str], primary: str
) -> tuple[str, tuple[bool, bool]]:
    """将 4 位数字转化为指定格式，并返回头部和尾部是否为 0"""
    result = ""

    if len(set(chunk)) == 1 and chunk[0] == 0:
        return result, (True, True)

    make_unit: Callable[[int], str] = lambda i: transer[chunk[i]] + suffix[i]

    zero = False
    if chunk[0] != 0:
        result = make_unit(0)
        zero = True

    if chunk[1] != 0:
        result += make_unit(1)
        zero = True
    elif zero and (chunk[2] != 0 or chunk[3] != 0):
        result += "零"
        zero = False

    if chunk[2] != 0:
        result += make_unit(2)
    elif zero and chunk[3] != 0:
        result += "零"

    if chunk[3] != 0:
        result += make_unit(3)

    result += primary
    return result, (chunk[0] == 0, chunk[3] == 0)


def _cut(some: list[int]) -> Generator[list[int], None, None]:
    """将给定 list 切分为长度为 4 的 list，逆序，不足补 0"""
    while len(some) > 4:
        yield some[-4:]
        some = some[:-4]

    yield [0] * (4 - len(some)) + some


def main() -> None:
    if len(sys.argv) != 2:
        _help(sys.argv[0])
        return

    result = convert(float(sys.argv[1]))
    if result:
        print(result)
        return

    print("转换失败")


def _help(name: str) -> None:
    print(f"用法: {name} <数字>")


if __name__ == "__main__":
    main()
