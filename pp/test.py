import unittest

from main import Kind, Node, Token, get_tree, parse, print_pritter, to_rpl


class TreeTest(unittest.TestCase):
    def setUp(self):
        pass

    def tearDown(self):
        pass

    def test_parse(self):
        actual = parse(
            'A AND BBB OR ("C  C" OR D AND E OR (F OR G AND H OR (I OR J AND K)))   L'
        )

        expected = [
            Token("(", Kind.Parenthesis),
            Token("A", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("BBB", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token('"C  C"', Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("D", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("E", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token("F", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("G", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("H", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token("I", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("J", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("K", Kind.Identifier),
            Token(")", Kind.Parenthesis),
            Token(")", Kind.Parenthesis),
            Token(")", Kind.Parenthesis),
            Token("AND", Kind.Operator),
            Token("L", Kind.Identifier),
            Token(")", Kind.Parenthesis),
        ]

        self.maxDiff = None
        self.assertEqual(expected, actual)

    def test_parse_and_and_or(self):
        actual = parse(
            'AND AND OR'
        )

        expected = [
            Token("(", Kind.Parenthesis),
            Token("AND", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("OR", Kind.Identifier),
            Token(")", Kind.Parenthesis),
        ]

        self.maxDiff = None
        self.assertEqual(expected, actual)

    def test_parse_a_or_b_or(self):
        actual = parse(
            'A OR (B OR)'
        )

        expected = [
            Token("(", Kind.Parenthesis),
            Token("A", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token("B", Kind.Identifier),
            Token(")", Kind.Parenthesis),
            Token(")", Kind.Parenthesis),
        ]

        self.maxDiff = None
        self.assertEqual(expected, actual)

    def test_parse_a_or_b_and(self):
        actual = parse(
            'A OR (B AND)'
        )

        expected = [
            Token("(", Kind.Parenthesis),
            Token("A", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token("B", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("AND", Kind.Identifier),
            Token(")", Kind.Parenthesis),
            Token(")", Kind.Parenthesis),
        ]

        self.maxDiff = None
        self.assertEqual(expected, actual)

    def test_parse_a_or_b_or_c(self):
        actual = parse(
            'A OR (B OR C)'
        )

        expected = [
            Token("(", Kind.Parenthesis),
            Token("A", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token("B", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("C", Kind.Identifier),
            Token(")", Kind.Parenthesis),
            Token(")", Kind.Parenthesis),
        ]

        self.maxDiff = None
        self.assertEqual(expected, actual)

    def test_to_rpl(self):
        tokens = [
            Token("(", Kind.Parenthesis),
            Token("A", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("(", Kind.Parenthesis),
            Token("B", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token("C", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("D", Kind.Identifier),
            Token(")", Kind.Parenthesis),
            Token("OR", Kind.Operator),
            Token("E", Kind.Identifier),
            Token(")", Kind.Parenthesis),
        ]

        expected = [
            Token("A", Kind.Identifier),
            Token("B", Kind.Identifier),
            Token("C", Kind.Identifier),
            Token("D", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("OR", Kind.Operator),
            Token("AND", Kind.Operator),
            Token("E", Kind.Identifier),
            Token("OR", Kind.Operator),
        ]

        actual = to_rpl(tokens)

        for i, a in enumerate(actual):
            self.assertEqual(expected[i].kind, a.kind)
            self.assertEqual(expected[i].word, a.word)

    def test_get_tree(self):
        tokens = [
            Token("(", Kind.Parenthesis),
            Token("A", Kind.Identifier),
            Token("B", Kind.Identifier),
            Token("C", Kind.Identifier),
            Token("D", Kind.Identifier),
            Token("AND", Kind.Operator),
            Token("OR", Kind.Operator),
            Token("AND", Kind.Operator),
            Token("E", Kind.Identifier),
            Token("OR", Kind.Operator),
            Token(")", Kind.Parenthesis),
        ]

        node_1 = Node("A", Kind.Identifier)
        node_2 = Node("B", Kind.Identifier)
        node_3 = Node("C", Kind.Identifier)
        node_4 = Node("D", Kind.Identifier)
        node_5 = Node("AND", Kind.Operator, None, node_3, node_4)
        node_6 = Node("OR", Kind.Operator, None, node_1, node_5)
        node_7 = Node("AND", Kind.Operator)
        node_8 = Node("E", Kind.Identifier)
        node_9 = Node("OR", Kind.Operator, None, node_7, node_8)

        node_8.parent = node_9

        expected = node_9
        actual = get_tree(tokens)

        print_pritter(actual)

        self.assertEqual("OR", actual.value)
        self.assertEqual(Kind.Operator, actual.kind)

        self.assertEqual("AND", actual.lhs.value)
        self.assertEqual(Kind.Operator, actual.lhs.kind)

        self.assertEqual("E", actual.rhs.value)
        self.assertEqual(Kind.Identifier, actual.rhs.kind)
        self.assertIsNone(actual.rhs.lhs)
        self.assertIsNone(actual.rhs.rhs)

        actual = actual.lhs

        self.assertEqual("A", actual.lhs.value)
        self.assertEqual(Kind.Identifier, actual.lhs.kind)
        self.assertIsNone(actual.lhs)
        self.assertIsNone(actual.rhs)
        
    
