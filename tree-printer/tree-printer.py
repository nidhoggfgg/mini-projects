from typing import Optional, Any


class Tree(object):
    """a example tree printer

    Args:
        tree (Optional[list[Any]], optional): the list include the tree.
    """

    # the result of already generated str, not include \n.
    _result: list[str] = []

    # maybe use a list to simulate a tree is odd, but keep it simple :)
    def __init__(self, tree: Optional[list[Any]] = None) -> None:
        self.tree = tree

    def _make_tree(self, tree: list[Any], prefix: str = "") -> None:
        """recursive make the line of the tree"""

        # make the child nodes not include the last one
        for node in tree[:-1]:
            self._result.append(f"{prefix}├── {node}")
            if isinstance(node, list):
                self._make_tree(node, f"{prefix}│   ")

        # last child node is special, not only itself, but also next prefix
        self._result.append(f"{prefix}└── {tree[-1]}")
        if isinstance(tree[-1], list):
            self._make_tree(tree[-1], f"{prefix}    ")

    def make_tree(self) -> str:
        """make tree, return the str of result, include \n"""
        # make sure self.tree isn't None
        if not self.tree:
            return ""

        # the root node
        self._result.append(f"{self.tree}")

        # use recursive function make the full tree
        self._make_tree(self.tree)
        return "\n".join(self._result)

    def print(self) -> None:
        """make tree then print"""
        # make sure self.tree isn't None
        if not self.tree:
            return

        # the root node
        self._result.append(f"{self.tree}")

        self._make_tree(self.tree)
        for line in self._result:
            print(line)

    # TODO(nidhoggfgg): impl a push mathod (even a littie odd)


def main() -> None:
    some = [[1, [2], [3, 4]], [4]]
    tree = Tree(some)
    tree.print()


if __name__ == "__main__":
    main()
