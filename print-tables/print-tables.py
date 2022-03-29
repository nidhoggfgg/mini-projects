from enum import Enum


class Align(Enum):
    """Align type
    LEFT: align left.
    CENTER: align center.
    RIGHT: align right.
    """

    LEFT = 1
    CENTER = 2
    RIGHT = 3


class Seg(Enum):
    """segmentation
    NO: not add any segmentation line.
    ONLY_HEADER: add segmentation line between header and the first row.
    FULL: add segmentation line between all two adjacent line.
    """

    NO = 1
    ONLY_HEADER = 2
    FULL = 3


class Table(object):
    """make and print table

    Args:
        seg (Seg, optional):
            segmentation. see the main function output for more information. default Seg.ONLY_HEADER.
        seg_char (tuple[str, ...], optional):
            char used in made segmentation. default to ("─", "┼").
        dec (bool, optional):
            decoration. the border of table. default to False.
        dec_char (tuple[str, ...], optional):
            the char used to make up border. default to ("┌", "┐", "└", "┘", "┬", "┴", "├", "┤", "─", "│").
        align (Align, optional):
            align type. default to Align.LEFT.
        sep (str, optional):
            the char used to separate col. default to "│".
    """

    # headers in table
    _header: list[str] = []

    # rows in table
    _rows: list[list[str]] = []

    # the max width of each column
    _max_widths: list[int] = []

    # already generated segmentation line
    _maked_seg: str = ""

    # the result of already generated table
    _result: list[str] = []

    def __init__(
        self,
        seg: Seg = Seg.ONLY_HEADER,
        seg_char: tuple[str, ...] = ("─", "┼"),
        dec: bool = False,
        dec_char: tuple[str, ...] = (
            "┌",
            "┐",
            "└",
            "┘",
            "┬",
            "┴",
            "├",
            "┤",
            "─",
            "─",
            "│",
            "│",
        ),
        align: Align = Align.LEFT,
        sep: str = "│",
    ) -> None:
        self.seg = seg
        self.seg_char = seg_char
        self.sep = sep
        self.dec = dec
        self.dec_char = dec_char
        self.align = align

    def _re_count_width(self, row: list[str]) -> None:
        """regenerate of the max width of each column"""
        if not self._max_widths:
            self._max_widths = [len(v) + 3 for v in row]
            return

        self._max_widths = [
            max(len(row[i]), width - 3) + 3 for i, width in enumerate(self._max_widths)
        ]

    def _make_line(self, line: list[str]) -> str:
        """make one line not include border"""

        def gen_line(line: list[str], sep: str, align: str, widths: list[int]) -> str:
            items = [f"{col:{align}{width}}" for col, width in zip(line, widths)]
            return f"{sep}".join(items)

        if self.align is Align.LEFT:
            return gen_line(line, self.sep, "<", self._max_widths)

        if self.align is Align.CENTER:
            return gen_line(line, self.sep, "^", self._max_widths)

        if self.align is Align.RIGHT:
            return gen_line(line, self.sep, ">", self._max_widths)

        return ""

    def _dec(self) -> None:
        """make table with border"""
        self._no_dec()

        # l: left, r: right, t: top, b: bottom, h: horizon, v: vertical
        #           default = "┌", "┐", "└", "┘", "┬", "┴", "├", "┤", "─", "─", "│", "│"
        # dc: tuple[str; 8] =  lt,  rt,  lb,  rb,  tm,  bm,  lm,  rm,  th,  bh,  lv,  rv
        #             index =  0,   1,   2,   3,   4,   5,   6,   7,   8,   9,   10,  11
        dc = self.dec_char

        result = []

        # add top
        items = [f"{dc[8]*width}" for width in self._max_widths]
        top = f"{dc[4]}".join(items)
        result.append(f"{dc[0]}{top}{dc[1]}")

        for line in self._result:
            if not line:
                continue
            if line[0] == self.seg_char[0]:
                result.append(f"{dc[6]}{line}{dc[7]}")
                continue
            result.append(f"{dc[10]}{line}{dc[11]}")

        # add bottom
        items = [f"{dc[9]*width}" for width in self._max_widths]
        bottom = f"{dc[5]}".join(items)
        result.append(f"{dc[2]}{bottom}{dc[3]}")

        self._result = result

    def _no_dec(self) -> None:
        """make table not with border"""

        # the segmentation line is not made up, so make it
        self._make_seg()

        self._result = []
        self._result.append(self._make_line(self._header))
        self._add_seg(True)
        for row in self._rows:
            self._result.append(self._make_line(row))
            self._add_seg()

        # segmentation line is be made extra one
        if self.seg is Seg.FULL:
            self._result.pop()

    def _add_seg(self, is_header: bool = False) -> None:
        """add segmentation line to table"""
        if is_header and self.seg is Seg.ONLY_HEADER:
            self._result.append(self._maked_seg)
            return

        if self.seg is Seg.FULL:
            self._result.append(self._maked_seg)

    def _make_seg(self) -> None:
        """make segmentation line"""
        items = [f"{self.seg_char[0]*width}" for width in self._max_widths]
        self._maked_seg = f"{self.seg_char[1]}".join(items)

    def set_rows(self, rows: list[list[str]]) -> None:
        """set rows to table"""
        self._rows = []
        for row in rows:
            self.push(row)

    def set_header(self, header: list[str]) -> None:
        """set header to table"""
        self._header = header
        self._re_count_width(header)

    def push(self, row: list[str]) -> None:
        """push row to table"""
        if len(row) != len(self._header):
            return

        self._rows.append(row)
        self._re_count_width(row)

    def push_col(self, header: str, col: list[str]) -> None:
        """push column to table"""
        self._header.append(header)

        for (v, row) in zip(col, self._rows):
            row.append(v)

        max_width = len(header)
        for v in col:
            if len(v) > max_width:
                max_width = len(v)

        self._max_widths.append(max_width + 3)

    def make_table(self) -> list[str]:
        """make table"""
        if self.dec:
            self._dec()
        else:
            self._no_dec()
        return self._result

    def print(self) -> None:
        """make table and print it"""
        if self.dec:
            self._dec()
        else:
            self._no_dec()

        for line in self._result:
            print(line)


def main() -> None:
    header = ["author", "time", "bonus"]
    rows = [
        ["nidhoggfgg", "20222-03-22", "$500"],
        ["chan", "2021-08-31", "$1000"],
        ["Dasha", "2021-03-17", "$750"],
    ]

    table = Table()
    table.set_header(header)
    table.set_rows(rows)
    print("default:")
    table.print()

    print("\nAlign & Segmentation & Border:")
    table.align = Align.CENTER
    table.seg = Seg.FULL
    table.dec = True
    table.print()

    print("\nmore:")
    table.seg = Seg.ONLY_HEADER
    table.sep = " "
    table.seg_char = ("═", "═")
    table.dec = False
    table.print()

    print()
    table.push_col("some", ["1000", "2000", "3000"])
    table.seg_char = ("┄", "┄")
    table.dec_char = ("╭", "╖", "╰", "╝", "─", "═", "├", "╢", "─", "═", "│", "║")
    table.dec = True
    table.print()


if __name__ == "__main__":
    main()
