import math
from typing import Callable, Generator


class Coder(object):
    """de(en)code the hidden text

    Args:
        text (str):
            the text to decode or encode.
        chiper (tuple[str, str, str]):
            the str used to hide the hidden text.
            There are few character can't see in common (ZWC):
                1. U+200B (zero width space)
                2. U+200C (zero width non-joiner)
                3. U+200D (zero width joiner)
                ...
    """

    # temporary storage de(en)code result
    _result: list[str] = []

    # temporary storage decoded text (no SWC)
    _decoded: list[str] = []

    def __init__(
        self, text: str, chiper: tuple[str, str, str] = ("\u200b", "\u200c", "\u200d")
    ) -> None:
        self.text = text
        self.chiper = chiper

    def encode(self, hidden: str) -> str:
        """encode the hidden text to plaintext"""

        # clean the _result and check text length.
        self._clean()
        if len(self.text) < 2:
            print("the text is too short! no space the add the hidden info!")
            return ""

        # split the encoded hidden text to a special list
        parts = self._split_to_list(hidden)

        # this is a pythonic way to combine two lists
        # the [""] is for type checker, in normal time, [None] also works!
        self._result = [""] * (len(parts) + len(self.text))
        end_num = 2 * len(parts)
        self._result[:end_num:2] = self.text[:len(parts)]
        self._result[1:end_num:2] = parts
        self._result[end_num:] = self.text[len(parts):]

        return "".join(self._result)

    def decode(self) -> tuple[str, str]:
        """decode the hiddent text from text"""
        self._clean()
        tmp: list[str] = []
        for c in self.text:
            if c == self.chiper[0]:
                tmp.append("0")
            elif c == self.chiper[1]:
                tmp.append("1")
            elif c == self.chiper[2]:
                some = self._decode_hidden(tmp)
                self._result.append(some)
                tmp = []
            else:
                self._decoded.append(c)
        return "".join(self._decoded), "".join(self._result)

    def _clean(self) -> None:
        """clean the temporary result"""
        self._result = []
        self._decoded = []

    def _split_to_list(self, hidden: str) -> list[str]:
        """encode and split hidden text to a list"""
        # encode the hidden text to ZWC
        encoded = self._encode_hidden(hidden)

        # calculate the every parts size (for split).
        part_size = math.ceil(len(encoded) / (len(self.text) - 1))

        # split it
        to_split = self.cut_up(encoded)
        return list(to_split(part_size))

    def _decode_hidden(self, hidden: list[str]) -> str:
        """decode the hidden text"""
        some = "".join(hidden)
        some = some.replace(self.chiper[0], "0").replace(self.chiper[1], "1")
        return chr(int(some, 2))

    def _encode_hidden(self, hidden: str) -> str:
        """encode the hidden text"""
        result = []
        for c in hidden:
            # bin(ord(c)) will return something like '0b1100001', use [2:] to slice the '1100001'
            tmp = bin(ord(c))[2:]

            # replace the 0 to the ZWC
            tmp = tmp.replace("0", self.chiper[0]).replace("1", self.chiper[1])

            result.append(tmp)
            # append the end symbol of a char
            result.append(self.chiper[2])

        return "".join(result)

    @classmethod
    def cut_up(cls, text: str) -> Callable[[int], Generator[str, None, None]]:
        """
        a function help cut up str to list,
        every members in list has a fixed length
        """
        # wrap _cut in a function looks odd,
        # but it is a simple example for learn closure :)
        def _cut(num: int) -> Generator[str, None, None]:
            some = text
            while len(some) > num:
                yield some[:num]
                some = some[num:]
            yield some

        return _cut


def main() -> None:
    coder = Coder("hide some text in this sentence but invisible!")
    encoded = coder.encode("Hello world!")
    print(f"encode result: {encoded}\n")
    coder = Coder(encoded)
    a, b = coder.decode()
    print(f"decode the hidden text:\n  ├─plaintext: {a}\n  └─hidden text: {b}")


if __name__ == "__main__":
    main()
