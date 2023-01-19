import sys


def read_lines(file_name):
    """read file every line to a list"""
    with open(file_name, "r") as f:
        all_txt = f.read()
        lines = all_txt.split("\n")
        if lines[-1] == lines[-2] and lines[-1] == "":
            lines.pop()

        return lines


def get_max_widths(lines):
    """find the max width of a list"""
    return max([len(line) for line in lines])


def gen_header(title, max_widths, frame_char):
    """generate the frame's head"""
    left, right, h, v = frame_char
    header = []
    header1 = f"{left}{h*(max_widths+5)}{right}"
    header2 = f"{v} {title:{max_widths-3}} - □ x {v}"
    header.append(header1)
    header.append(header2)
    return header


def gen_frame(lines, max_widths, frame_char):
    """generate framed str"""
    # l: left, r: right, t: top, b: bottom, h: horizon, v: vertical
    lt, rt, lb, rb, h, v = frame_char
    max_widths += 5
    framed = []
    frame1 = f"{lt}{h*max_widths}{rt}"
    framed.append(frame1)

    for line in lines:
        framed_line = f"{v} {line:<{max_widths-1}}{v}"
        framed.append(framed_line)

    frame2 = f"{lb}{h*max_widths}{rb}"
    framed.append(frame2)
    return framed


def main():
    if len(sys.argv) != 2:
        print(f"Usage: {sys.argv[0]} <file>")
        sys.exit(1)
    lines = read_lines(sys.argv[1])
    max_widths = get_max_widths(lines)

    for line in gen_header("some txt", max_widths, ("┌", "┐", "─", "│")):
        print(line)

    for line in gen_frame(lines, max_widths, ("├", "┤", "└", "┘", "─", "│")):
        print(line)


if __name__ == "__main__":
    main()
