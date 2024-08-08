from __future__ import annotations
import pprint

from dataclasses import dataclass, field
from enum import Enum
from typing import Iterator, cast
from collections import deque


class Kind(Enum):
    Identifier = 1
    Operator = 2
    Parenthesis = 3


@dataclass
class Token:
    word: str
    kind: Kind


@dataclass
class Node:
    value: str
    kind: Kind
    parent: Node | None = None
    lhs: Node | None = None
    rhs: Node | None = None


def parse(expr: str):
    TOKEN_LIST = [" ", "(", ")"]
    OP_LIST = ["AND", "OR"]
    PA_LIST = ["(", ")"]

    tokens = []
    token = ""
    quote = False

    for c in f"({expr})":
        if not quote and (c in TOKEN_LIST):
            if len(token) > 0:
                tokens.append(token)
                token = ""

            if c in PA_LIST:
                tokens.append(c)

            continue

        if c == '"':
            quote = not quote

        token += c

    if len(token) > 0:
        tokens.append(token)

    tl: list[Token] = []
    for t in tokens:
        word = t.strip()
        if t in PA_LIST:
            if t == ")":
                last = tl[-1]

                if last.kind == Kind.Operator:
                    if last.word == "AND":
                        tl.append(Token("AND", Kind.Identifier))

                    if last.word == "OR":
                        tl.pop()

            tl.append(Token(word, Kind.Parenthesis))
            continue

        last = tl[-1]
        if t in OP_LIST:
            kind = Kind.Identifier if last.kind != Kind.Identifier else Kind.Operator
            tl.append(Token(word, kind))
            continue

        if last.kind == Kind.Identifier or last.word == ")":
            tl.append(Token("AND", Kind.Operator))

        tl.append(Token(word, Kind.Identifier))

    return tl


def to_rpl(tokens: list[Token]) -> list[Token]:
    priority_map = {
        "AND": 2,
        "OR": 1,
        "(": 0,
        ")": 0,
    }

    result: list[Token] = []
    stack: deque[Token] = deque([])

    for t in tokens:
        match t.kind:
            case Kind.Identifier:
                result.append(t)
            case Kind.Operator:
                while stack and priority_map.get(
                    stack[-1].word, -1
                ) >= priority_map.get(t.word, -1):
                    op = stack.pop()
                    if op.kind == Kind.Parenthesis:
                        continue

                    result.append(op)

                stack.append(t)
            case Kind.Parenthesis:
                if t.word == "(":
                    stack.append(t)

                if t.word == ")":
                    while True:
                        s = stack.pop()
                        if s.kind == Kind.Parenthesis and s.word == "(":
                            break
                        result.append(s)

    return result


def get_tree(tokens: list[Token]) -> Node | None:
    root: Node | None = None
    stack: deque[Node] = deque([])

    for t in tokens:
        match t.kind:
            case Kind.Identifier:
                stack.append(Node(t.word, t.kind))
            case Kind.Operator:
                node = Node(t.word, t.kind)
                node.rhs = stack.pop()
                node.lhs = stack.pop()

                node.rhs.parent = node
                node.lhs.parent = node

                root = node

                stack.append(node)

    return root


def print_pritter(node: Node | None, level: int = 0):
    if not node:
        return

    print("    " * level, end="")
    print(node.value, " (", node.kind, ")", sep="")
    match node.kind:
        case Kind.Operator:
            if node.lhs:
                print_pritter(node.lhs, level + 1)

            if node.rhs:
                print_pritter(node.rhs, level + 1)

            return
        case Kind.Identifier:
            return


if __name__ == "__main__":
    parse('A AND "B B" OR (C AND (D OR E AND F) OR G)')
