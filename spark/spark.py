#!/usr/bin/env python
import sys


def spark(values: list[str]) -> None:
    numbers = [float(v) for v in values]

    max_number = max(numbers)
    min_number = min(numbers)

    ticks: tuple[str, ...] = ("▁", "▂", "▃", "▄", "▅", "▆", "▇", "█")
    if max_number - min_number <= 0.00001:
        ticks = ("▅", "▆")

    f = (max_number - min_number) / (len(ticks) - 1)
    f = max(f, 1)

    for n in numbers:
        tmp = int((n - min_number) / f)
        print(ticks[tmp], end="")
    print()


def _help() -> None:
    print(f"Usage: {sys.argv[0]} [-h|--help] [values, ...]")


def main() -> None:
    if len(sys.argv) == 1:
        _help()
        sys.exit(2)

    if sys.argv[1] == "-h" or sys.argv[1] == "--help":
        _help()
        sys.exit(0)

    spark(sys.argv[1:])


if __name__ == "__main__":
    main()
