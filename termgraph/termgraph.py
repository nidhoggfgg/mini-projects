import csv
from math import ceil, floor
from typing import Sequence


def render_at(
    data: tuple[float, ...],
    height: int,
    y_axis: int,
    max_value: float,
    min_value: float,
) -> str:
    """render the char of the given y_axis"""
    top_stick = to_height(data[1], height, max_value, min_value)
    bottom_stick = to_height(data[2], height, max_value, min_value)

    top_candle = max(data[0], data[3])
    bottom_candle = min(data[0], data[3])
    top_candle = to_height(top_candle, height, max_value, min_value)
    bottom_candle = to_height(bottom_candle, height, max_value, min_value)

    # simplified, easy but not good.
    # if ceil(top_candle) <= h <= ceil(top_stick):
    #     return "│"
    # elif floor(bottom_candle) <= h <= ceil(top_candle):
    #     return "┃"
    # elif floor(bottom_stick) <= h <= floor(bottom_candle):
    #     return "│"
    # else:
    #     return " "

    # a little huge, but pretty good.
    # the top stick.
    if floor(top_candle) <= y_axis <= ceil(top_stick):
        if top_candle - y_axis > 0.75:
            return "┃"

        if top_candle - y_axis > 0.25:
            if top_stick - y_axis > 0.75:
                return "╽"
            return "╻"

        if top_stick - y_axis > 0.75:
            return "│"

        if top_stick - y_axis > 0.25:
            return "╷"

        return " "

    # the body
    if ceil(bottom_candle) <= y_axis <= floor(top_candle):
        return "┃"

    # the bottom stick
    if floor(bottom_stick) <= y_axis <= ceil(bottom_candle):
        if bottom_candle - y_axis < 0.25:
            return "┃"

        if bottom_candle - y_axis < 0.75:
            if bottom_stick - y_axis < 0.25:
                return "╿"
            return "╹"

        if bottom_stick - y_axis < 0.25:
            return "│"

        if bottom_stick - y_axis < 0.75:
            return "╵"
        return " "

    # nor the stick and body
    return " "


def draw(datas: Sequence[tuple[float, ...]], height: int) -> None:
    """draw the graph of the datas with the height of `height`"""
    max_value = max([data[1] for data in datas])
    min_value = min([data[2] for data in datas])
    result = ""
    for y in reversed(range(0, height)):
        for data in datas:
            result += render_at(data, height, y, max_value, min_value)
        result += "\n"
    print(result)


def to_height(data: float, height: int, max_value: float, min_value: float) -> float:
    """translate the data to a height"""
    return (data - min_value) / (max_value - min_value) * height


def main() -> None:
    with open("./test.csv") as f:
        reader = csv.reader(f, delimiter=",")
        next(reader)
        datas = [(float(a), float(b), float(c), float(d)) for a, b, c, d in reader]
    print(datas[2])
    draw(datas, 30)


if __name__ == "__main__":
    main()
