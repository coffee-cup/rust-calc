# rust-calc

A simple calculator implemented in rust.

## Usage

``` shell
$ cargo run
> 1
1
> 2 + 2
4
> 3 * (2 + -4) ^ 4
48
```

## Implementation

The calculator is implemented as a lexer, parser, and interpreter. The parser is
LR(1) and uses [Pratt's algorithm](https://en.wikipedia.org/wiki/Pratt_parser)
for parsing operator precedence. I found [these
resources](https://pinboard.in/u:jakerunzer/t:pratt/) really useful when
learning about and implementing this type of parser.
